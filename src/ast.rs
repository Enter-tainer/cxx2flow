use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum AstNode {
    Dummy,
    Stat(String),
    // Content
    Continue(String),
    // Content
    Break(String),
    // Content
    Return(String),
    // Content
    If(String, Vec<Rc<RefCell<Ast>>>, Vec<Rc<RefCell<Ast>>>),
    // Conditon, Children1, Children2
    While(String, Vec<Rc<RefCell<Ast>>>),
    // Condtion, Children
    DoWhile(String, Vec<Rc<RefCell<Ast>>>),
    // Condition, Children
}
#[derive(Debug)]
pub struct Ast {
    pub id: usize,
    pub node: AstNode,
    pub fa: Option<Weak<RefCell<Ast>>>,
    pub next: Option<Weak<RefCell<Ast>>>,
    pub prev: Option<Weak<RefCell<Ast>>>,
}

impl Ast {
    pub fn new(id: &mut usize, node: AstNode) -> Ast {
        *id = *id + 1;
        Ast {
            id: *id - 1,
            node,
            fa: None,
            next: None,
            prev: None,
        }
    }

    pub fn is_loop(&self) -> bool {
        match self.node {
            AstNode::While(_, _) | AstNode::DoWhile(_, _) => true,
            _ => false,
        }
    }
}
