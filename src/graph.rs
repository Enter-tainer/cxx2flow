use std::{cell::RefCell, rc::Rc};

use crate::ast::{Ast, AstNode};
use anyhow::Result;
#[derive(Debug)]
pub enum GraphNodeType {
    Start,
    // next
    End,
    Node(Option<usize>),
    // optional jump, otherwise, it is next node
    Choice(Option<usize>, Option<usize>),
    // true, false
}

impl GraphNodeType {
    fn set_jump(&mut self, target: usize) -> Result<()> {
        if let GraphNodeType::Node(ref mut t) = self {
            *t = Some(target);
            Ok(())
        } else {
            Err(anyhow::anyhow!("cannot set jump for node \"{:?}\"", self))
        }
    }
    fn set_true_branch(&mut self, target: usize) -> Result<()> {
        if let GraphNodeType::Choice(ref mut t, _) = self {
            *t = Some(target);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "cannot set true branch for node \"{:?}\"",
                self
            ))
        }
    }
    fn set_false_branch(&mut self, target: usize) -> Result<()> {
        if let GraphNodeType::Choice(_, ref mut f) = self {
            *f = Some(target);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "cannot set false branch for node \"{:?}\"",
                self
            ))
        }
    }
}

#[derive(Debug)]
pub struct GraphNode {
    pub id: usize,
    pub node_type: GraphNodeType,
    pub content: String,
}

pub type Graph = Vec<GraphNode>;

type AstMap = Vec<usize>;

fn build_ast_map_single(
    ast: &Rc<RefCell<Ast>>,
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<()> {
    let ast_id = ast.borrow().id;
    let id = graph.len();
    ast_map[ast_id] = id;
    match &ast.borrow().node {
        AstNode::Dummy => Err(anyhow::anyhow!("unexpected dummy node")),
        AstNode::Stat(s) | AstNode::Continue(s) | AstNode::Break(s) | AstNode::Return(s) => {
            graph.push(GraphNode {
                id,
                node_type: GraphNodeType::Node(None),
                content: s.clone(),
            });
            Ok(())
        }
        AstNode::If(cond, t, f) => {
            graph.push(GraphNode {
                id,
                node_type: GraphNodeType::Choice(None, None),
                content: cond.clone(),
            });
            build_ast_map_vec(t, graph, ast_map)?;
            build_ast_map_vec(f, graph, ast_map)?;
            Ok(())
        }
        AstNode::While(cond, body) => {
            graph.push(GraphNode {
                id,
                node_type: GraphNodeType::Choice(None, None),
                content: cond.clone(),
            });
            build_ast_map_vec(body, graph, ast_map)?;
            Ok(())
        }
        AstNode::DoWhile(cond, body) => {
            graph.push(GraphNode {
                id,
                node_type: GraphNodeType::Choice(None, None),
                content: cond.clone(),
            });
            build_ast_map_vec(body, graph, ast_map)?;
            Ok(())
        }
    }
}

fn build_ast_map_vec(
    ast: &[Rc<RefCell<Ast>>],
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<()> {
    for i in ast {
        build_ast_map_single(i, graph, ast_map)?;
    }
    Ok(())
}

fn build_ast_map(ast: &[Rc<RefCell<Ast>>], graph: &mut Graph, ast_map: &mut AstMap) -> Result<()> {
    graph.push(GraphNode {
        id: 0,
        node_type: GraphNodeType::Start,
        content: String::from("Start"),
    });
    build_ast_map_vec(ast, graph, ast_map)?;
    graph.push(GraphNode {
        id: graph.len(),
        node_type: GraphNodeType::End,
        content: String::from("End"),
    });
    Ok(())
}

fn find_nearest_loop(
    ast: Rc<RefCell<Ast>>,
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<Option<usize>> {
    let mut i = Rc::downgrade(&ast);
    loop {
        match (*i.upgrade().unwrap()).borrow().fa.as_ref() {
            Some(f) => {
                let t = i.upgrade().unwrap();
                i = f.clone();
                if t.borrow().is_loop() {
                    return Ok(Some(ast_map[t.borrow().id]));
                }
            }
            None => return Ok(None),
        }
    }
}

fn find_nearest_loop_break(
    ast: Rc<RefCell<Ast>>,
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<Option<usize>> {
    let mut i = Rc::downgrade(&ast);
    loop {
        match (*i.upgrade().unwrap()).borrow().fa.as_ref() {
            Some(f) => {
                let t = i.upgrade().unwrap();
                i = f.clone();
                // now t is the first loop
                if t.borrow().is_loop() {
                    if let Some(next) = &t.borrow().next {
                        // if loop have next, return next's id
                        return Ok(Some(ast_map[next.upgrade().unwrap().borrow().id]));
                    };
                    // or we will climb up to find next
                    let mut j = Rc::downgrade(&t);
                    loop {
                        match (*j.upgrade().unwrap()).borrow().fa.as_ref() {
                            Some(f) => {
                                let t = j.upgrade().unwrap();
                                j = f.clone();
                                if let Some(t) = &t.borrow().next {
                                    return Ok(Some(ast_map[t.upgrade().unwrap().borrow().id]));
                                };
                            }
                            None => return Ok(None),
                        }
                    }
                }
            }
            None => return Ok(None),
        }
    }
}

fn find_next_exec(
    ast: Rc<RefCell<Ast>>,
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<Option<usize>> {
    // Assuming that this node has no siblings...
    let node = ast.borrow();
    if let Some(n) = &node.next {
        return Ok(Some(ast_map[n.upgrade().unwrap().borrow().id]));
    }
    if let Some(f) = &node.fa {
        let fa = f.upgrade().unwrap();
        if fa.borrow().is_loop() {
            Ok(Some(ast_map[fa.borrow().id]))
        } else {
            let next = find_next_exec(fa, graph, ast_map)?;
            Ok(next)
        }
    } else {
        Ok(None)
    }
}

fn build_graph_single(
    ast: Rc<RefCell<Ast>>,
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<()> {
    let node = ast.borrow();
    let id = ast_map[node.id];
    let len = graph.len();
    match &node.node {
        AstNode::Dummy => Err(anyhow::anyhow!("unexpected dummy node")),
        AstNode::Stat(_) => match node.next {
            Some(_) => Ok(()),
            None => {
                let next = find_next_exec(ast.clone(), graph, ast_map)?;
                graph[id].node_type.set_jump(next.unwrap_or(len - 1))?;
                Ok(())
            }
        },
        AstNode::Continue(_) => {
            let next = find_nearest_loop(ast.clone(), graph, ast_map)?;
            graph[id].node_type.set_jump(next.unwrap_or(len - 1))?;
            Ok(())
        }
        AstNode::Break(_) => {
            let next = find_nearest_loop_break(ast.clone(), graph, ast_map)?;
            graph[id].node_type.set_jump(next.unwrap_or(len - 1))?;
            Ok(())
        }
        AstNode::Return(_) => graph[id].node_type.set_jump(len - 1),
        AstNode::If(_, b1, b2) => {
            if !b1.is_empty() {
                let b1_first_id = b1[0].borrow().id;
                graph[id].node_type.set_true_branch(ast_map[b1_first_id])?;
            } else {
                let next = find_next_exec(ast.clone(), graph, ast_map)?;
                graph[id]
                    .node_type
                    .set_true_branch(next.unwrap_or(len - 1))?;
            }
            if !b2.is_empty() {
                let b2_first_id = b2[0].borrow().id;
                graph[id].node_type.set_false_branch(ast_map[b2_first_id])?;
            } else {
                let next = find_next_exec(ast.clone(), graph, ast_map)?;
                graph[id]
                    .node_type
                    .set_false_branch(next.unwrap_or(len - 1))?;
            }
            build_graph_vec(b1, graph, ast_map)?;
            build_graph_vec(b2, graph, ast_map)?;
            Ok(())
        }
        AstNode::While(_, body) => {
            if !body.is_empty() {
                let body_first_id = body[0].borrow().id;
                graph[id]
                    .node_type
                    .set_true_branch(ast_map[body_first_id])?;
            }
            let next = find_next_exec(ast.clone(), graph, ast_map)?;
            graph[id]
                .node_type
                .set_false_branch(next.unwrap_or(len - 1))?;

            // set false branch: find next stat, similar to previous
            build_graph_vec(&body, graph, ast_map)
        }
        AstNode::DoWhile(_, body) => {
            if !body.is_empty() {
                let body_first_id = body[0].borrow().id;
                graph[id]
                    .node_type
                    .set_true_branch(ast_map[body_first_id])?;
            }
            let next = find_nearest_loop_break(ast.clone(), graph, ast_map)?;
            graph[id]
                .node_type
                .set_false_branch(next.unwrap_or(len - 1))?;
            build_graph_vec(&body, graph, ast_map)
        }
    }
}

fn build_graph_vec(
    ast: &[Rc<RefCell<Ast>>],
    graph: &mut Graph,
    ast_map: &mut AstMap,
) -> Result<()> {
    for i in ast {
        build_graph_single(i.clone(), graph, ast_map)?;
    }
    Ok(())
}

fn build_graph(ast: &[Rc<RefCell<Ast>>], graph: &mut Graph, ast_map: &mut AstMap) -> Result<()> {
    build_graph_vec(ast, graph, ast_map)
}

pub fn from_ast(ast: Vec<Rc<RefCell<Ast>>>, max_id: usize) -> Result<Graph> {
    let mut ast_map: AstMap = vec![0; max_id];
    let mut graph: Graph = Vec::new();
    build_ast_map(&ast, &mut graph, &mut ast_map)?;
    build_graph(&ast, &mut graph, &mut ast_map)?;
    Ok(graph)
}
