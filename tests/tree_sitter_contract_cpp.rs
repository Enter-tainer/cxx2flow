use std::{fs, path::PathBuf};

use tree_sitter::{Node, Parser};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("syntax_contract_cpp.cpp")
}

fn read_fixture() -> Vec<u8> {
    fs::read(fixture_path()).expect("failed to read syntax_contract_cpp.cpp")
}

fn parse_tree(content: &[u8]) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_cpp::language())
        .expect("failed to set tree-sitter cpp language");
    parser
        .parse(content, None)
        .expect("failed to parse C++ contract fixture")
}

fn first_child_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    if node.kind() == kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if let Some(found) = first_child_by_kind(cursor.node(), kind) {
                return Some(found);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

#[test]
fn tree_sitter_cpp_specific_kinds_exist() {
    let content = read_fixture();
    let tree = parse_tree(&content);
    let root = tree.root_node();

    for kind in [
        "namespace_definition",
        "template_declaration",
        "class_specifier",
        "field_declaration_list",
        "lambda_expression",
        "for_range_loop",
        "try_statement",
        "catch_clause",
        "enum_specifier",
        "qualified_identifier",
    ] {
        assert!(
            first_child_by_kind(root, kind).is_some(),
            "expected to find C++ specific node kind `{kind}`"
        );
    }
}

#[test]
fn tree_sitter_cpp_required_fields_exist() {
    let content = read_fixture();
    let tree = parse_tree(&content);
    let root = tree.root_node();

    let range_for = first_child_by_kind(root, "for_range_loop")
        .expect("missing for_range_loop in syntax_contract_cpp.cpp");
    assert!(
        range_for.child_by_field_name("type").is_some(),
        "for_range_loop should have `type` field"
    );
    assert!(
        range_for.child_by_field_name("declarator").is_some(),
        "for_range_loop should have `declarator` field"
    );
    assert!(
        range_for.child_by_field_name("right").is_some(),
        "for_range_loop should have `right` field"
    );
    assert!(
        range_for.child_by_field_name("body").is_some(),
        "for_range_loop should have `body` field"
    );

    let function = first_child_by_kind(root, "function_definition")
        .expect("missing function_definition in C++ contract fixture");
    assert!(
        function.child_by_field_name("declarator").is_some(),
        "function_definition should have `declarator` field"
    );
    assert!(
        function.child_by_field_name("body").is_some(),
        "function_definition should have `body` field"
    );

    let switch_stmt =
        first_child_by_kind(root, "switch_statement").expect("missing switch_statement node");
    assert!(
        switch_stmt.child_by_field_name("condition").is_some(),
        "switch_statement should have `condition` field"
    );
    assert!(
        switch_stmt.child_by_field_name("body").is_some(),
        "switch_statement should have `body` field"
    );
}
