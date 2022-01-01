use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::ast::{Ast, AstNode};
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

pub fn parse(content: &[u8], function_name: Option<String>) -> Result<Rc<RefCell<Ast>>> {
    let mut parser = Parser::new();
    let language = tree_sitter_cpp::language();
    parser.set_language(language)?;
    let tree = parser.parse(&content, None).unwrap();
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
        let stats = cursor.node().child_by_field_name("body").unwrap();
        let node = cursor.node().child_by_field_name("declarator");
        if node.is_none() {
            return Err(Error::NotFound("declarator"));
        }
        let node = node.unwrap();
        let func_name = filter_ast(node, "identifier");
        if func_name.is_none() {
            continue;
        }
        let func_name = func_name.unwrap().utf8_text(content).unwrap();
        if func_name != target_function {
            continue;
        }
        let mut id: usize = 0;
        let res = parse_stat(&mut id, stats, content)?;
        remove_dummy(res.clone());
        set_links(res.clone(), None);
        return Ok(res);
    }
    Err(Error::NotFound("target function"))
}

fn remove_dummy(ast: Rc<RefCell<Ast>>) {
    match &mut ast.borrow_mut().node {
        AstNode::If {
            cond,
            body,
            otherwise,
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

fn set_links(ast: Rc<RefCell<Ast>>, fa: Option<Weak<RefCell<Ast>>>) {
    ast.borrow_mut().fa = fa;
    match &ast.borrow_mut().node {
        AstNode::If {
            cond,
            body,
            otherwise,
        } => {
            set_links(body.clone(), Some(Rc::downgrade(&ast)));
            if let Some(b2) = otherwise {
                set_links(b2.clone(), Some(Rc::downgrade(&ast)));
            }
        }
        AstNode::While { body, .. }
        | AstNode::DoWhile { body, .. }
        | AstNode::For { body, .. }
        | AstNode::Switch { body, .. } => {
            set_links(body.clone(), Some(Rc::downgrade(&ast)));
        }
        AstNode::Compound(v) => {
            if v.len() >= 2 {
                for i in 1..v.len() {
                    let prev = &v[i - 1];
                    let next = &v[i];
                    prev.borrow_mut().next = Some(Rc::downgrade(next));
                    next.borrow_mut().prev = Some(Rc::downgrade(prev));
                }
            }
            for i in v {
                set_links(i.clone(), Some(Rc::downgrade(&ast)));
            }
        }
        _ => {}
    }
}

fn parse_stat(id: &mut usize, stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    match stat.kind() {
        "compound_statement" => {
            let mut cursor = stat.walk();
            let mut vec = Vec::new();
            if !cursor.goto_first_child() {
                return Ok(Rc::new(RefCell::new(Ast::new(
                    id,
                    AstNode::Compound(Vec::new()),
                    None,
                ))));
            }
            loop {
                let node = cursor.node();
                let ast = parse_stat(id, node, content)?;
                vec.push(ast);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            Ok(Rc::new(RefCell::new(Ast::new(
                id,
                AstNode::Compound(vec),
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
                    .unwrap()
                    .utf8_text(content)?;
                label_vec.push(label_str.to_owned());
                cursor.goto_first_child();
                while cursor.goto_next_sibling() {}
                if cursor.node().kind() != "labeled_statement" {
                    break;
                }
            }
            let ast = parse_stat(id, cursor.node(), content)?;
            ast.borrow_mut().label = Some(label_vec);
            Ok(ast)
        }
        _ => {
            let res = parse_single_stat(id, stat, content);
            if let Ok(res) = res {
                return Ok(res);
            }
            if let Err(msg) = res {
                if !matches!(msg, Error::GarbageToken(_)) {
                    return Err(msg);
                } else {
                    return Ok(Rc::new(RefCell::new(Ast::new(id, AstNode::Dummy, None))));
                }
            }
            unreachable!();
        }
    }
}

fn parse_single_stat(id: &mut usize, stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    match stat.kind() {
        "continue_statement" => Ok(Rc::new(RefCell::new(Ast::new(
            id,
            AstNode::Continue("continue".to_string()),
            None,
        )))),
        "break_statement" => Ok(Rc::new(RefCell::new(Ast::new(
            id,
            AstNode::Break("break".to_string()),
            None,
        )))),
        "return_statement" => {
            let str = stat.utf8_text(content)?;
            Ok(Rc::new(RefCell::new(Ast::new(
                id,
                AstNode::Return(String::from(str)),
                None,
            ))))
        }
        "if_statement" => parse_if_stat(id, stat, content),
        "while_statement" => parse_while_stat(id, stat, content),
        "do_statement" => parse_do_while_stat(id, stat, content),
        "for_statement" => parse_for_stat(id, stat, content),
        "switch_statement" => parse_switch_stat(id, stat, content),
        "goto_statement" => parse_goto_stat(id, stat, content),
        "expression_statement" | "declaration" => {
            let str = stat.utf8_text(content)?;
            Ok(Rc::new(RefCell::new(Ast::new(
                id,
                AstNode::Stat(String::from(str)),
                None,
            ))))
        }
        // ignore all unrecognized token
        c => Err(Error::GarbageToken(c)),
    }
}

fn parse_if_stat(id: &mut usize, if_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = if_stat.child_by_field_name("condition").unwrap();
    let blk1 = if_stat.child_by_field_name("consequence");
    let blk2 = if_stat.child_by_field_name("alternative");
    let cond_str = condition.utf8_text(content)?;
    let body = parse_stat(id, blk1.unwrap(), content)?;

    let otherwise = if blk2.is_some() {
        Some(parse_stat(id, blk2.unwrap(), content)?)
    } else {
        None
    };

    let res = Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::If {
            cond: String::from(cond_str),
            body,
            otherwise,
        },
        None,
    )));
    Ok(res)
}

fn parse_while_stat(id: &mut usize, while_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = while_stat.child_by_field_name("condition").unwrap();
    let body = while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let body = parse_stat(id, body.unwrap(), content)?;

    let res = Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::While {
            cond: String::from(cond_str),
            body,
        },
        None,
    )));
    Ok(res)
}

// return first child, or return the case label
fn get_case_child_and_label<'a>(
    mut case_stat: tree_sitter::TreeCursor<'a>,
    content: &[u8],
) -> (Option<TreeCursor<'a>>, String) {
    let label = String::from(if case_stat.node().child(0).unwrap().kind() == "case" {
        case_stat
            .node()
            .child(1)
            .unwrap()
            .utf8_text(content)
            .unwrap()
    } else {
        case_stat
            .node()
            .child(0)
            .unwrap()
            .utf8_text(content)
            .unwrap()
    });
    case_stat.goto_first_child();
    while case_stat.node().kind() != ":" {
        case_stat.goto_next_sibling();
    }

    (
        if case_stat.goto_next_sibling() {
            Some(case_stat)
        } else {
            None
        },
        label,
    )
}

fn parse_switch_stat(
    id: &mut usize,
    switch_stat: Node,
    content: &[u8],
) -> Result<Rc<RefCell<Ast>>> {
    let condition = switch_stat.child_by_field_name("condition").unwrap();
    let body = switch_stat.child_by_field_name("body").unwrap();
    let cond_str = condition.utf8_text(content)?;
    let mut vec_stat = Vec::new();
    let mut vec_label = Vec::new();
    let mut cursor = body.walk();
    cursor.goto_first_child(); // brace
    cursor.goto_next_sibling(); // case statement
                                // dbg!(cursor.node());
    loop {
        let (child, label) = get_case_child_and_label(cursor.clone(), content);
        vec_label.push(label);
        if let Some(child) = child {
            let mut cursor = child;
            let first_idx = vec_stat.len();
            loop {
                let stat = parse_stat(id, cursor.node(), content)?;
                vec_stat.push(stat);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            vec_stat[first_idx].borrow_mut().label = Some(vec_label.clone());
        }
        if !cursor.goto_next_sibling() {
            break;
        }
        if cursor.node().kind() != "case_statement" {
            break;
        }
    }
    let inner = Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::Compound(vec_stat),
        None,
    )));
    let res = Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::Switch {
            cond: String::from(cond_str),
            body: inner,
        },
        None,
    )));
    Ok(res)
}

fn parse_goto_stat(id: &mut usize, goto_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let label_str = goto_stat
        .child_by_field_name("label")
        .unwrap()
        .utf8_text(content)?;
    Ok(Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::Goto(label_str.to_owned()),
        None,
    ))))
}

fn parse_do_while_stat(
    id: &mut usize,
    do_while_stat: Node,
    content: &[u8],
) -> Result<Rc<RefCell<Ast>>> {
    let condition = do_while_stat.child_by_field_name("condition").unwrap();
    let body = do_while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let body = parse_stat(id, body.unwrap(), content)?;
    let res = Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::DoWhile {
            cond: String::from(cond_str),
            body,
        },
        None,
    )));

    Ok(res)
}

fn parse_for_stat(id: &mut usize, for_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
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
    let body = parse_stat(id, cursor.node(), content)?;
    let res = Rc::new(RefCell::new(Ast::new(
        id,
        AstNode::For {
            init: init_str,
            cond: cond_str,
            upd: update_str,
            body,
        },
        None,
    )));
    Ok(res)
}
