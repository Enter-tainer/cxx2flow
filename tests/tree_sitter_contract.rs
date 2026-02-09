use std::{fs, path::PathBuf};

use tree_sitter::{Node, Parser};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("syntax_contract.c")
}

fn read_fixture() -> Vec<u8> {
    fs::read(fixture_path()).expect("failed to read syntax_contract.c")
}

fn parse_tree(content: &[u8]) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
        .expect("failed to set tree-sitter cpp language");
    parser
        .parse(content, None)
        .expect("failed to parse fixture with tree-sitter")
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
fn tree_sitter_core_kinds_exist() {
    let content = read_fixture();
    let tree = parse_tree(&content);
    let root = tree.root_node();

    for kind in [
        "function_definition",
        "if_statement",
        "while_statement",
        "do_statement",
        "for_statement",
        "switch_statement",
        "goto_statement",
        "continue_statement",
        "break_statement",
        "return_statement",
        "labeled_statement",
        "case_statement",
    ] {
        assert!(
            first_child_by_kind(root, kind).is_some(),
            "expected to find node kind `{kind}` in syntax_contract.c"
        );
    }
}

#[test]
fn tree_sitter_required_fields_exist() {
    let content = read_fixture();
    let tree = parse_tree(&content);
    let root = tree.root_node();

    let function = first_child_by_kind(root, "function_definition")
        .expect("missing function_definition in syntax_contract.c");
    assert!(
        function.child_by_field_name("declarator").is_some(),
        "function_definition should have `declarator` field"
    );
    assert!(
        function.child_by_field_name("body").is_some(),
        "function_definition should have `body` field"
    );

    let if_stmt = first_child_by_kind(root, "if_statement").expect("missing if_statement node");
    assert!(
        if_stmt.child_by_field_name("condition").is_some(),
        "if_statement should have `condition` field"
    );
    assert!(
        if_stmt.child_by_field_name("consequence").is_some(),
        "if_statement should have `consequence` field"
    );

    let while_stmt =
        first_child_by_kind(root, "while_statement").expect("missing while_statement node");
    assert!(
        while_stmt.child_by_field_name("condition").is_some(),
        "while_statement should have `condition` field"
    );
    assert!(
        while_stmt.child_by_field_name("body").is_some(),
        "while_statement should have `body` field"
    );

    let do_stmt = first_child_by_kind(root, "do_statement").expect("missing do_statement node");
    assert!(
        do_stmt.child_by_field_name("condition").is_some(),
        "do_statement should have `condition` field"
    );
    assert!(
        do_stmt.child_by_field_name("body").is_some(),
        "do_statement should have `body` field"
    );

    let for_stmt = first_child_by_kind(root, "for_statement").expect("missing for_statement node");
    assert!(
        for_stmt.child_by_field_name("initializer").is_some(),
        "for_statement should have `initializer` field"
    );
    assert!(
        for_stmt.child_by_field_name("condition").is_some(),
        "for_statement should have `condition` field"
    );
    assert!(
        for_stmt.child_by_field_name("update").is_some(),
        "for_statement should have `update` field"
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

    let goto_stmt =
        first_child_by_kind(root, "goto_statement").expect("missing goto_statement node");
    assert!(
        goto_stmt.child_by_field_name("label").is_some(),
        "goto_statement should have `label` field"
    );
}
