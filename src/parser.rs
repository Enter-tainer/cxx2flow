use std::{cell::RefCell, rc::Rc};

use crate::ast::{Ast, AstNode};
#[allow(unused_imports)]
use crate::dump::dump_node;
use crate::error::{Error, Result};
use tree_sitter::{Node, Parser, TreeCursor};

fn filter_ast<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    if node.kind() == kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if let Some(v) = filter_ast(cursor.node(), kind) {
                return Some(v);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

pub fn parse(
    content: &[u8],
    _file_name: &str,
    function_name: Option<String>,
) -> Result<Rc<RefCell<Ast>>> {
    let mut parser = Parser::new();
    let language = tree_sitter_cpp::language();
    parser.set_language(language)?;
    let tree = parser
        .parse(&content, None)
        .ok_or(Error::TreesitterParseFailed)?;
    let mut cursor = tree.walk();
    cursor.goto_first_child();
    let mut functions: Vec<Node> = Vec::new();
    loop {
        let node = cursor.node();
        let node = filter_ast(node, "function_definition");
        if let Some(node) = node {
            functions.push(node);
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
    let target_function = function_name.unwrap_or_else(|| "main".to_string());
    for i in functions {
        cursor.reset(i);
        let stats = cursor
            .node()
            .child_by_field_name("body")
            .ok_or(Error::ChildNotFound)?;
        let node = cursor
            .node()
            .child_by_field_name("declarator")
            .ok_or(Error::DeclaratorNotFound)?;
        let func_name = filter_ast(node, "identifier");
        if func_name.is_none() {
            continue;
        }
        let func_name = func_name.unwrap().utf8_text(content)?;
        if func_name != target_function {
            continue;
        }
        let res = parse_stat(stats, content)?;
        remove_dummy(res.clone());
        return Ok(res);
    }
    Err(Error::FunctionNotFound {
        src: target_function.clone(),
        range: (0..target_function.len()).into(),
    })
}

fn remove_dummy(ast: Rc<RefCell<Ast>>) {
    match &mut ast.borrow_mut().node {
        AstNode::If {
            body, otherwise, ..
        } => {
            remove_dummy(body.clone());
            if let Some(otherwise) = otherwise {
                remove_dummy(otherwise.clone());
            }
        }
        AstNode::While { body, .. }
        | AstNode::DoWhile { body, .. }
        | AstNode::For { body, .. }
        | AstNode::Switch { body, .. } => {
            remove_dummy(body.clone());
        }
        AstNode::Compound(v) => {
            v.retain(|x| !matches!(x.borrow().node, AstNode::Dummy));
            v.iter().for_each(|x| {
                remove_dummy(x.clone());
            });
        }
        _ => {}
    }
}

fn parse_stat(stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    match stat.kind() {
        "compound_statement" => {
            let mut cursor = stat.walk();
            let mut vec = Vec::new();
            if !cursor.goto_first_child() {
                return Ok(Rc::new(RefCell::new(Ast::new(
                    AstNode::Compound(Vec::new()),
                    stat.byte_range(),
                    None,
                ))));
            }
            loop {
                let node = cursor.node();
                let ast = parse_stat(node, content)?;
                vec.push(ast);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            Ok(Rc::new(RefCell::new(Ast::new(
                AstNode::Compound(vec),
                stat.byte_range(),
                None,
            ))))
        }
        "labeled_statement" => {
            let mut label_vec = Vec::new();
            let mut cursor = stat.walk();
            loop {
                let node = cursor.node();
                let label_str = node
                    .child_by_field_name("label")
                    .ok_or(Error::ChildNotFound)?
                    .utf8_text(content)?;
                label_vec.push(label_str.to_owned());
                cursor.goto_first_child();
                while cursor.goto_next_sibling() {}
                if cursor.node().kind() != "labeled_statement" {
                    break;
                }
            }
            let ast = parse_stat(cursor.node(), content)?;
            ast.borrow_mut().label = Some(label_vec);
            Ok(ast)
        }
        _ => {
            let res = parse_single_stat(stat, content);
            match res {
                Ok(res) => Ok(res),
                Err(msg) => {
                    if !matches!(msg, Error::GarbageToken(_)) {
                        Err(msg)
                    } else {
                        Ok(Rc::new(RefCell::new(Ast::new(
                            AstNode::Dummy,
                            stat.byte_range(),
                            None,
                        ))))
                    }
                }
            }
        }
    }
}

fn parse_single_stat(stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    match stat.kind() {
        "continue_statement" => Ok(Rc::new(RefCell::new(Ast::new(
            AstNode::Continue("continue".to_string()),
            stat.byte_range(),
            None,
        )))),
        "break_statement" => Ok(Rc::new(RefCell::new(Ast::new(
            AstNode::Break("break".to_string()),
            stat.byte_range(),
            None,
        )))),
        "return_statement" => {
            let str = stat.utf8_text(content)?;
            Ok(Rc::new(RefCell::new(Ast::new(
                AstNode::Return(String::from(str)),
                stat.byte_range(),
                None,
            ))))
        }
        "if_statement" => parse_if_stat(stat, content),
        "while_statement" => parse_while_stat(stat, content),
        "do_statement" => parse_do_while_stat(stat, content),
        "for_statement" => parse_for_stat(stat, content),
        "switch_statement" => parse_switch_stat(stat, content),
        "goto_statement" => parse_goto_stat(stat, content),
        "expression_statement" | "declaration" => {
            let str = stat.utf8_text(content)?;
            Ok(Rc::new(RefCell::new(Ast::new(
                AstNode::Stat(String::from(str)),
                stat.byte_range(),
                None,
            ))))
        }
        // ignore all unrecognized token
        c => Err(Error::GarbageToken(c)),
    }
}

fn parse_if_stat(if_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = if_stat
        .child_by_field_name("condition")
        .ok_or(Error::ChildNotFound)?;
    let blk1 = if_stat.child_by_field_name("consequence");
    let blk2 = if_stat.child_by_field_name("alternative");
    let cond_str = condition.utf8_text(content)?;
    let body = parse_stat(blk1.ok_or(Error::ChildNotFound)?, content)?;

    let otherwise = if blk2.is_some() {
        Some(parse_stat(blk2.unwrap(), content)?)
    } else {
        None
    };

    let res = Rc::new(RefCell::new(Ast::new(
        AstNode::If {
            cond: String::from(cond_str),
            body,
            otherwise,
        },
        if_stat.byte_range(),
        None,
    )));
    Ok(res)
}

fn parse_while_stat(while_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = while_stat
        .child_by_field_name("condition")
        .ok_or(Error::ChildNotFound)?;
    let body = while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let body = parse_stat(body.ok_or(Error::ChildNotFound)?, content)?;

    let res = Rc::new(RefCell::new(Ast::new(
        AstNode::While {
            cond: String::from(cond_str),
            body,
        },
        while_stat.byte_range(),
        None,
    )));
    Ok(res)
}

/// return first child, or return the case label
fn get_case_child_and_label<'a>(
    mut case_stat: tree_sitter::TreeCursor<'a>,
    content: &[u8],
) -> Result<(Option<TreeCursor<'a>>, String)> {
    // dump_node(&case_stat.node(), None);
    let label = {
        let tmp = if case_stat
            .node()
            .child(0)
            .ok_or(Error::ChildNotFound)?
            .kind()
            == "case"
        {
            case_stat
                .node()
                .child(1)
                .ok_or(Error::ChildNotFound)?
                .utf8_text(content)?
        } else {
            case_stat
                .node()
                .child(0)
                .ok_or(Error::ChildNotFound)?
                .utf8_text(content)?
        };
        tmp.into()
    };
    case_stat.goto_first_child();
    if case_stat.node().kind() == "case" {
        // case lit :
        case_stat.goto_next_sibling();
        case_stat.goto_next_sibling();
    } else if case_stat.node().kind() == "default" {
        // default :
        case_stat.goto_next_sibling();
    }
    while [":", "comment"].contains(&case_stat.node().kind()) {
        if !case_stat.goto_next_sibling() {
            return Ok((None, label));
        }
    }
    // dump_node(&case_stat.node(), None);
    Ok((Some(case_stat), label))
}

fn parse_switch_stat(switch_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = switch_stat
        .child_by_field_name("condition")
        .ok_or(Error::ChildNotFound)?;
    let body = switch_stat
        .child_by_field_name("body")
        .ok_or(Error::ChildNotFound)?;
    let cond_str = condition.utf8_text(content)?;
    let mut stats = Vec::new();
    let mut labels = Vec::new();
    let mut cases = Vec::new();
    let mut cursor = body.walk();
    cursor.goto_first_child(); // brace
    cursor.goto_next_sibling(); // case statement
                                // dbg!(cursor.node());
    loop {
        let (child, label) = get_case_child_and_label(cursor.clone(), content)?;
        labels.push(label.clone());
        cases.push(label);
        if let Some(child) = child {
            let mut cursor = child;
            let first_idx = stats.len();
            loop {
                let stat = parse_stat(cursor.node(), content)?;
                stats.push(stat);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            stats[first_idx].borrow_mut().label = Some(labels.clone());
            labels.clear();
        }
        if !cursor.goto_next_sibling() {
            break;
        }
        if cursor.node().kind() != "case_statement" {
            break;
        }
    }
    let inner = Rc::new(RefCell::new(Ast::new(
        AstNode::Compound(stats),
        switch_stat.byte_range(),
        None,
    )));
    let res = Rc::new(RefCell::new(Ast::new(
        AstNode::Switch {
            cond: String::from(cond_str),
            cases,
            body: inner,
        },
        switch_stat.byte_range(),
        None,
    )));
    Ok(res)
}

fn parse_goto_stat(goto_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let label_str = goto_stat
        .child_by_field_name("label")
        .ok_or(Error::ChildNotFound)?
        .utf8_text(content)?;
    Ok(Rc::new(RefCell::new(Ast::new(
        AstNode::Goto(label_str.to_owned()),
        goto_stat.byte_range(),
        None,
    ))))
}

fn parse_do_while_stat(do_while_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = do_while_stat
        .child_by_field_name("condition")
        .ok_or(Error::ChildNotFound)?;
    let body = do_while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let body = parse_stat(body.ok_or(Error::ChildNotFound)?, content)?;
    let res = Rc::new(RefCell::new(Ast::new(
        AstNode::DoWhile {
            cond: String::from(cond_str),
            body,
        },
        do_while_stat.byte_range(),
        None,
    )));

    Ok(res)
}

fn parse_for_stat(for_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let mut cursor = for_stat.walk();
    let init = for_stat.child_by_field_name("initializer");
    let cond = for_stat.child_by_field_name("condition");
    let update = for_stat.child_by_field_name("update");
    let mut init_str: String = String::new();
    let mut cond_str: String = String::from("true");
    let mut update_str: String = String::new();
    if let Some(init) = init {
        let init = init.utf8_text(content)?;
        init_str = String::from(init);
    }
    if let Some(cond) = cond {
        let cond = cond.utf8_text(content)?;
        cond_str = String::from(cond);
    }
    if let Some(update) = update {
        let update = update.utf8_text(content)?;
        update_str = String::from(update);
    }
    cursor.goto_first_child();
    while cursor.goto_next_sibling() {}
    let body = parse_stat(cursor.node(), content)?;
    let res = Rc::new(RefCell::new(Ast::new(
        AstNode::For {
            init: init_str,
            cond: cond_str,
            upd: update_str,
            body,
        },
        for_stat.byte_range(),
        None,
    )));
    Ok(res)
}
