use anyhow::Result;
use tree_sitter::{Node, Parser, TreeCursor};

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
        return parse_compond_stat_to_ast(stats);
    }
    unreachable!();
}

fn parse_compond_stat_to_ast(compond_stat: Node) -> Result<Vec<Ast>> {
    let mut cursor = compond_stat.walk();
    let mut vec: Vec<Ast> = Vec::new();
    if !cursor.goto_first_child() {
        return Ok(vec);
    }
    loop {

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    Ok(vec)
}