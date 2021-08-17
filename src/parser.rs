use std::vec;

use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::ast::Ast;

pub fn parse(path: &str) -> Result<Vec<Ast>> {
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
        if node.kind() == "function_definition" {
            functions.push(node);
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
    for i in functions {
        cursor.reset(i);
        let stats = cursor.node().child_by_field_name("body").unwrap();
        return parse_stat(stats, &content);
    }
    unreachable!();
}

fn parse_stat(stat: Node, content: &[u8]) -> Result<Vec<Ast>> {
    info!("{:?}", stat);
    if stat.kind() == "compound_statement" {
        let mut cursor = stat.walk();
        let mut vec: Vec<Ast> = Vec::new();
        if !cursor.goto_first_child() {
            return Ok(vec);
        }
        loop {
            let mut skip = false;
            let mut inner_cursor = cursor.clone();
            loop {
                let node = inner_cursor.node();
                // info!("node: {:?}, kind: {:?}", node, node.kind());
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
            let node = parse_single_stat(cursor.node(), content);
            if let Ok(node) = node {
                vec.push(node);
            } else if let Err(msg) = node {
                if msg.to_string() != "garbage token" {
                    return Err(msg);
                }
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        Ok(vec)
    } else {
        let res = parse_single_stat(stat, content);
        if let Ok(res) = res {
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

fn parse_single_stat(stat: Node, content: &[u8]) -> Result<Ast> {
    info!("parsing single stat: {:?}", stat);
    match stat.kind() {
        "continue_statement" => Ok(Ast::Continue("continue".to_string())),
        "break_statement" => Ok(Ast::Break("break".to_string())),
        "return_statement" => {
            let str = stat.utf8_text(content)?;
            Ok(Ast::Return(String::from(str)))
        }
        "if_statement" => parse_if_stat(stat, content),
        "for_statement" => parse_for_stat(stat, content),
        "while_statement" => parse_while_stat(stat, content),
        "expression_statement" | "declaration" => {
            let str = stat.utf8_text(content)?;
            Ok(Ast::Stat(String::from(str)))
        }
        // ignore all unrecognized token
        _ | "{" | "}" | "comment" => Err(anyhow::anyhow!("garbage token")),
        // _ => Err(anyhow::format_err!(
        //     "unknown statement: {:?}, kind: {:?}",
        //     stat,
        //     stat.kind()
        // )),
    }
    // unreachable!();
}

fn parse_if_stat(if_stat: Node, content: &[u8]) -> Result<Ast> {
    let condition = if_stat.child_by_field_name("condition").unwrap();
    let blk1 = if_stat.child_by_field_name("consequence");
    let blk2 = if_stat.child_by_field_name("alternative");
    let cond_str = condition.utf8_text(content)?;
    let blk1 = if blk1.is_some() {
        parse_stat(blk1.unwrap(), content)?
    } else {
        vec![]
    };
    let blk2 = if blk2.is_some() {
        parse_stat(blk2.unwrap(), content)?
    } else {
        vec![]
    };
    let res: Ast = Ast::If(String::from(cond_str), blk1, blk2);
    Ok(res)
}

fn parse_while_stat(while_stat: Node, content: &[u8]) -> Result<Ast> {
    let condition = while_stat.child_by_field_name("condition").unwrap();
    let body = while_stat.child_by_field_name("body");
    let cond_str = condition.utf8_text(content)?;
    let body = if body.is_some() {
        parse_stat(body.unwrap(), content)?
    } else {
        vec![]
    };
    let res: Ast = Ast::While(String::from(cond_str), body);
    Ok(res)
}

fn parse_for_stat(for_stat: Node, content: &[u8]) -> Result<Ast> {
    let mut cursor = for_stat.walk();
    let init = for_stat.child_by_field_name("initializer");
    let cond = for_stat.child_by_field_name("condition");
    let update = for_stat.child_by_field_name("update");
    let mut init_str: String = String::new();
    let mut cond_str: String = String::new();
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
    let mut body: Vec<Ast> = vec![];
    if cursor.goto_first_child() {
        while cursor.goto_next_sibling() {}
        body = parse_stat(cursor.node(), content)?;
    };
    let res: Ast = Ast::For(init_str, cond_str, update_str, body);
    Ok(res)
}
