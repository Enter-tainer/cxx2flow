use std::{cell::RefCell, ops::Range, rc::Rc, collections::HashSet};

use tree_sitter::Node;

#[derive(Debug)]
pub struct Expr {
    pub string: String,
    pub func_call: HashSet<String>,
}

#[derive(Debug)]
pub enum AstNode {
    Dummy,
    /// Children
    Compound(Vec<Rc<RefCell<Ast>>>),
    /// Content
    Stat(Expr),
    /// Content
    Continue(String),
    /// Content
    Break(String),
    /// Content
    Return(Expr),
    /// Condition, Children1, Children2
    If {
        cond: Expr,
        body: Rc<RefCell<Ast>>,
        otherwise: Option<Rc<RefCell<Ast>>>,
    },
    /// Condition, Children
    While {
        cond: Expr,
        body: Rc<RefCell<Ast>>,
    },
    /// Condition, Children
    DoWhile {
        cond: Expr,
        body: Rc<RefCell<Ast>>,
    },
    /// Init, Condition, Update, Children
    For {
        init: Expr,
        cond: Expr,
        upd: Expr,
        body: Rc<RefCell<Ast>>,
    },
    /// Condition, Children, Body
    Switch {
        cond: Expr,
        cases: Vec<String>,
        body: Rc<RefCell<Ast>>,
    },
    /// Label Name
    Goto(String),
}
#[derive(Debug)]
pub struct Ast {
    pub node: AstNode,
    pub range: Range<usize>,
    pub label: Option<Vec<String>>,
}

impl Ast {
    pub fn new(node: AstNode, range: Range<usize>, label: Option<Vec<String>>) -> Ast {
        Ast { node, range, label }
    }
}

pub fn filter_ast_first<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    if node.kind() == kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if let Some(v) = filter_ast_first(cursor.node(), kind) {
                return Some(v);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

fn filter_ast_impl<'a>(node: Node<'a>, kind: &str, buffer: &mut Vec<Node<'a>>) {
    if node.kind() == kind {
        buffer.push(node);
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            filter_ast_impl(cursor.node(), kind, buffer);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

pub fn filter_ast<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
    let mut buffer = Vec::new();
    filter_ast_impl(node, kind, &mut buffer);
    buffer
}
