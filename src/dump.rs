use tree_sitter::Node;
#[allow(dead_code)]
fn dump_node_internal(node: &Node, level: usize) {
    println!("{}{:?}", " ".repeat(level), node);
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            dump_node_internal(&cursor.node(), level + 2);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

#[allow(dead_code)]
pub fn dump_node(node: &Node, msg: Option<&str>) {
    println!("===={}====", msg.unwrap_or_default());
    dump_node_internal(node, 0);
}
