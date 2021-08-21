use std::{cell::RefCell, rc::Rc, vec};

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::ast::{Ast, AstNode};

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

pub fn parse(path: &str, function_name: Option<String>) -> Result<(Vec<Rc<RefCell<Ast>>>, usize)> {
    let mut parser = Parser::new();
    let language = tree_sitter_cpp::language();
    parser.set_language(language)?;
    let content = std::fs::read(path)?;
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
    let target_function = function_name.unwrap_or("main".to_string());
    for i in functions {
        cursor.reset(i);
        let stats = cursor.node().child_by_field_name("body").unwrap();
        let node = cursor.node().child_by_field_name("declarator");
        if node.is_none() {
            return Err(anyhow::anyhow!("declarator not found in node"));
        }
        let node = node.unwrap();
        let func_name = filter_ast(node, "identifier");
        if func_name.is_none() {
            continue;
        }
        let func_name = func_name.unwrap().utf8_text(&content).unwrap();
        if func_name != target_function {
            continue;
        }
        let mut id: usize = 0;
        let res = parse_stat(&mut id, stats, None, &content)?;
        return Ok((res, id));
    }
    Err(anyhow::anyhow!(
        "function \"{}\" not found in this file.",
        target_function
    ))
}

fn parse_stat(
    id: &mut usize,
    stat: Node,
    fa: Option<Rc<RefCell<Ast>>>,
    content: &[u8],
) -> Result<Vec<Rc<RefCell<Ast>>>> {
    match stat.kind() {
        "compound_statement" => {
            let mut cursor = stat.walk();
            let mut vec: Vec<Rc<RefCell<Ast>>> = Vec::new();
            if !cursor.goto_first_child() {
                return Ok(vec);
            }
            loop {
                let mut skip = false;
                let mut inner_cursor = cursor.clone();
                loop {
                    let node = inner_cursor.node();
                    if node.kind() == "compound_statement" {
                        if !inner_cursor.goto_first_child() {
                            skip = true
                        }
                    } else {
                        break;
                    }
                }
                if skip {
                    continue;
                }
                if cursor.node().kind() == "for_statement" {
                    let node = parse_for_stat(id, cursor.node(), fa.clone(), content);
                    if let Ok(node) = node {
                        for i in node {
                            vec.push(i);
                        }
                    } else if let Err(msg) = node {
                        if msg.to_string() != "garbage token" {
                            return Err(msg);
                        }
                    }
                } else {
                    let node = parse_single_stat(id, cursor.node(), content);
                    if let Ok(node) = node {
                        vec.push(node);
                    } else if let Err(msg) = node {
                        if msg.to_string() != "garbage token" {
                            return Err(msg);
                        }
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            for i in &vec {
                i.borrow_mut().fa = fa.as_ref().map(|fa| Rc::downgrade(fa));
            }
            if vec.len() >= 2 {
                for i in 1..vec.len() {
                    vec[i].borrow_mut().prev = Some(Rc::downgrade(&vec[i - 1]));
                    vec[i - 1].borrow_mut().next = Some(Rc::downgrade(&vec[i]));
                }
            }
            Ok(vec)
        }
        "for_statement" => {
            let mut vec: Vec<Rc<RefCell<Ast>>> = Vec::new();
            let node = parse_for_stat(id, stat, fa.clone(), content);
            if let Ok(node) = node {
                for i in node {
                    vec.push(i);
                }
            } else if let Err(msg) = node {
                if msg.to_string() != "garbage token" {
                    return Err(msg);
                }
            }
            Ok(vec)
        }
        _ => {
            let res = parse_single_stat(id, stat, content);
            if let Ok(res) = res {
                res.borrow_mut().fa = fa.map(|fa| Rc::downgrade(&fa));
                return Ok(vec![res]);
            }
            if let Err(msg) = res {
                if msg.to_string() != "garbage token" {
                    return Err(msg);
                } else {
                    return Ok(vec![]);
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
        )))),
        "break_statement" => Ok(Rc::new(RefCell::new(Ast::new(
            id,
            AstNode::Break("break".to_string()),
        )))),
        "return_statement" => {
            let str = stat.utf8_text(content)?;
            Ok(Rc::new(RefCell::new(Ast::new(
                id,
                AstNode::Return(String::from(str)),
            ))))
        }
        "if_statement" => parse_if_stat(id, stat, content),
        "while_statement" => parse_while_stat(id, stat, content),
        "do_statement" => parse_do_while_stat(id, stat, content),
        "expression_statement" | "declaration" => {
            let str = stat.utf8_text(content)?;
            Ok(Rc::new(RefCell::new(Ast::new(
                id,
                AstNode::Stat(String::from(str)),
            ))))
        }
        // ignore all unrecognized token
        _ => Err(anyhow::anyhow!("garbage token")),
        // _ | "{" | "}" | "comment" => Err(anyhow::anyhow!("garbage token")),
        // _ => Err(anyhow::format_err!(
        //     "unknown statement: {:?}, kind: {:?}",
        //     stat,
        //     stat.kind()
        // )),
    }
    // unreachable!();
}

fn parse_if_stat(id: &mut usize, if_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = if_stat.child_by_field_name("condition").unwrap();
    let blk1 = if_stat.child_by_field_name("consequence");
    let blk2 = if_stat.child_by_field_name("alternative");
    let cond_str = condition.utf8_text(content)?;
    let res = Rc::new(RefCell::new(Ast::new(id, AstNode::Dummy)));
    let blk1 = if blk1.is_some() {
        parse_stat(id, blk1.unwrap(), Some(res.clone()), content)?
    } else {
        vec![]
    };
    let blk2 = if blk2.is_some() {
        parse_stat(id, blk2.unwrap(), Some(res.clone()), content)?
    } else {
        vec![]
    };
    res.borrow_mut().node = AstNode::If(String::from(cond_str), blk1, blk2);
    Ok(res)
}

fn parse_while_stat(id: &mut usize, while_stat: Node, content: &[u8]) -> Result<Rc<RefCell<Ast>>> {
    let condition = while_stat.child_by_field_name("condition").unwrap();
    let body = while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let res = Rc::new(RefCell::new(Ast::new(id, AstNode::Dummy)));
    let body = if body.is_some() {
        parse_stat(id, body.unwrap(), Some(res.clone()), content)?
    } else {
        vec![]
    };
    res.borrow_mut().node = AstNode::While(String::from(cond_str), body);
    Ok(res)
}

fn parse_do_while_stat(
    id: &mut usize,
    do_while_stat: Node,
    content: &[u8],
) -> Result<Rc<RefCell<Ast>>> {
    let condition = do_while_stat.child_by_field_name("condition").unwrap();
    let body = do_while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let res = Rc::new(RefCell::new(Ast::new(id, AstNode::Dummy)));
    let body = if body.is_some() {
        parse_stat(id, body.unwrap(), Some(res.clone()), content)?
    } else {
        vec![]
    };
    res.borrow_mut().node = AstNode::DoWhile(String::from(cond_str), body);
    Ok(res)
}

fn parse_for_stat(
    id: &mut usize,
    for_stat: Node,
    fa: Option<Rc<RefCell<Ast>>>,
    content: &[u8],
) -> Result<Vec<Rc<RefCell<Ast>>>> {
    let mut cursor = for_stat.walk();
    let init = for_stat.child_by_field_name("initializer");
    let cond = for_stat.child_by_field_name("condition");
    let update = for_stat.child_by_field_name("update");
    let mut init_str: String = String::new();
    let mut cond_str: String = String::from("true");
    let mut update_str: String = String::new();
    let res = Rc::new(RefCell::new(Ast::new(id, AstNode::Dummy)));
    let mut res_vec: Vec<Rc<RefCell<Ast>>> = Vec::new();
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
    let mut body: Vec<Rc<RefCell<Ast>>> = vec![];
    if cursor.goto_first_child() {
        while cursor.goto_next_sibling() {}
        body = parse_stat(id, cursor.node(), Some(res.clone()), content)?;
    };
    if !update_str.is_empty() {
        body.push(Rc::new(RefCell::new(Ast::new(
            id,
            AstNode::Stat(update_str),
        ))));
        body.last().unwrap().borrow_mut().fa = Some(Rc::downgrade(&res));
        if body.len() >= 2 {
            body[body.len() - 2].borrow_mut().next = Some(Rc::downgrade(body.last().unwrap()));
        }
    }
    if !init_str.is_empty() {
        res_vec.push(Rc::new(RefCell::new(Ast::new(id, AstNode::Stat(init_str)))));
    }
    res.borrow_mut().node = AstNode::While(cond_str, body);
    res.borrow_mut().is_for = true;
    res_vec.push(res);
    for i in &res_vec {
        i.borrow_mut().fa = fa.as_ref().map(|fa| Rc::downgrade(fa));
    }
    if res_vec.len() == 2 {
        res_vec[0].borrow_mut().next = Some(Rc::downgrade(&res_vec[1]));
        res_vec[1].borrow_mut().prev = Some(Rc::downgrade(&res_vec[0]));
    }
    Ok(res_vec)
}
