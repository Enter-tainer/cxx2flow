use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum AstNode {
    Dummy,
    Compound(Vec<Rc<RefCell<Ast>>>),
    // Children
    Stat(String),
    // Content
    Continue(String),
    // Content
    Break(String),
    // Content
    Return(String),
    // Content
    If {
        cond: String,
        body: Rc<RefCell<Ast>>,
        otherwise: Option<Rc<RefCell<Ast>>>,
    },
    // Conditon, Children1, Children2
    While {
        cond: String,
        body: Rc<RefCell<Ast>>,
    },
    // Condtion, Children
    DoWhile {
        cond: String,
        body: Rc<RefCell<Ast>>,
    },
    // Condition, Children
    For {
        init: String,
        cond: String,
        upd: String,
        body: Rc<RefCell<Ast>>,
    },
    // Init, Condition, Update, Children
    Switch {
        cond: String,
        body: Rc<RefCell<Ast>>,
    },
    // Condition, Children
    Goto(String),
    // Label Name
}
#[derive(Debug)]
pub struct Ast {
    pub id: usize, // TODO: can be removed?
    pub node: AstNode,
    pub label: Option<Vec<String>>,
    pub fa: Option<Weak<RefCell<Ast>>>,
    pub next: Option<Weak<RefCell<Ast>>>,
    pub prev: Option<Weak<RefCell<Ast>>>,
}

impl Ast {
    pub fn new(id: &mut usize, node: AstNode, label: Option<Vec<String>>) -> Ast {
        *id += 1;
        Ast {
            id: *id - 1,
            node,
            fa: None,
            next: None,
            prev: None,
            label,
        }
    }

    pub fn is_loop(&self) -> bool {
        matches!(
            self.node,
            AstNode::While { .. } | AstNode::DoWhile { .. } | AstNode::For { .. }
        )
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
        | AstNode::Return(_)
        | AstNode::Goto(_) => None,
        AstNode::If { body, otherwise, .. } => {
            let b1 = filter_id(body.clone(), id);
            if b1.is_some() {
                return b1;
            }
            if let Some(b2) = otherwise {
                return filter_id(b2.clone(), id);
            }
            None
        }
        AstNode::While { body, .. }
        | AstNode::DoWhile { body, .. }
        | AstNode::For { body, .. }
        | AstNode::Switch { body, .. } => filter_id(body.clone(), id),
        AstNode::Compound(b) => {
            for i in b {
                if let Some(v) = filter_id(i.clone(), id) {
                    return Some(v);
                }
            }
            None
        }
    }
}
