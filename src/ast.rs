use std::rc::Rc;

use uuid::Uuid;
pub enum Ast {
    Stat(String),
    // Content
    Continue(String),
    // Content
    Break(String),
    // Content
    Return(String),
    // Content
    If(String, Vec<Ast>, Vec<Ast>),
    // Conditon, Children1, Children2
    While(String, Vec<Ast>),
    // Condtion, Children
    For(String, String, String, Vec<Ast>),
    // Init, Cond, Upd, Children
}
