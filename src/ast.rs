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
    pub is_for: bool,
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
            is_for: false,
        }
    }

    pub fn is_loop(&self) -> bool {
        match self.node {
            AstNode::While(_, _) | AstNode::DoWhile(_, _) => true,
            _ => false,
        }
    }
}

pub fn filter_id(node: Rc<RefCell<Ast>>, id: usize) -> Option<Rc<RefCell<Ast>>> {
    if node.borrow().id == id {
        return Some(node);
    }
    match &node.borrow().node {
        AstNode::Dummy
        | AstNode::Stat(_)
        | AstNode::Continue(_)
        | AstNode::Break(_)
        | AstNode::Return(_) => None,
        AstNode::If(_, b1, b2) => {
            for i in b1 {
                if let Some(t) = filter_id(i.clone(), id) {
                    return Some(t);
                }
            }
            for i in b2 {
                if let Some(t) = filter_id(i.clone(), id) {
                    return Some(t);
                }
            }
            None
        }
        AstNode::DoWhile(_, b) | AstNode::While(_, b) => {
            for i in b {
                if let Some(t) = filter_id(i.clone(), id) {
                    return Some(t);
                }
            }
            None
        }
    }
}
