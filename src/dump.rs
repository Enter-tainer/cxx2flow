use std::collections::HashMap;

use colored::*;
use itertools::Itertools;
use tree_sitter::{Node, TreeCursor};

pub fn useful_children<'a, 'tree>(
    node: &'a Node<'tree>,
    cursor: &'a mut TreeCursor<'tree>,
) -> impl Iterator<Item = Node<'tree>> + 'a {
    node.children(cursor).enumerate().filter_map(move |(i, n)| {
        if n.is_named() || node.field_name_for_child(i as u32).is_some() {
            Some(n)
        } else {
            None
        }
    })
}

#[allow(dead_code)]
fn dump_node_internal(
    node: &Node,
    prefix: &str,
    content: &[u8],
    field_name: Option<&str>,
    is_last: bool,
    is_init: bool,
) {
    let node_text = node.utf8_text(content).unwrap();
    let start = node.start_position();
    let end = node.end_position();
    let kind = node.kind();
    println!(
        "{}{}{}: `{}` {} - {}{}",
        prefix,
        if is_init {
            ""
        } else if is_last {
            "└──"
        } else {
            "├──"
        },
        match field_name {
            Some(name) => name.bold().yellow(),
            None => "[ANON]".normal(),
        },
        kind.bold(),
        start,
        end,
        if node.child_count() == 0 || !node_text.contains('\n') {
            format!(" {} {}", "->".cyan(), node_text.bold()).bold()
        } else {
            "".to_owned().normal()
        }
    );
    let node_to_idx: HashMap<_, _> = {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .enumerate()
            .map(|(x, y)| (y, x))
            .collect()
    };
    let nodes: Vec<_> = {
        let mut cursor = node.walk();
        // useful_children(node, &mut cursor).collect_vec()
        node.children(&mut cursor).collect_vec()
    };
    let prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
    for (pos, i) in nodes.into_iter().with_position() {
        match pos {
            itertools::Position::First | itertools::Position::Middle => {
                dump_node_internal(
                    &i,
                    &prefix,
                    content,
                    node.field_name_for_child(node_to_idx[&i] as u32),
                    false,
                    false,
                );
            }
            itertools::Position::Last | itertools::Position::Only => {
                dump_node_internal(
                    &i,
                    &prefix,
                    content,
                    node.field_name_for_child(node_to_idx[&i] as u32),
                    true,
                    false,
                );
            }
        }
    }
}

#[allow(dead_code)]
pub fn dump_node(node: &Node, content: &[u8]) {
    dump_node_internal(node, "", content, None, true, true);
}
