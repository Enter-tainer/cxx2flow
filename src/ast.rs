use std::{cell::RefCell, rc::Rc};

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
    // Condition, Children1, Children2
    While {
        cond: String,
        body: Rc<RefCell<Ast>>,
    },
    // Condition, Children
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
    pub node: AstNode,
    pub label: Option<Vec<String>>,
}

impl Ast {
    pub fn new(node: AstNode, label: Option<Vec<String>>) -> Ast {
        Ast { node, label }
    }
}
