use crate::ast::{Ast, AstNode};
use crate::error::{Error, Result};
use itertools::Itertools;
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use petgraph::EdgeDirection;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum GraphNodeType {
    Dummy, // dummy nodes will be removed eventually
    Begin,
    End,
    Node(String),
    Choice(String),
}

#[derive(Debug, Clone, Copy)]
pub enum EdgeType {
    Normal,
    Branch(bool),
}

pub type Graph = StableDiGraph<GraphNodeType, EdgeType>;

struct GraphContext {
    pub graph: Graph,
    pub break_target: Option<NodeIndex>,
    pub continue_target: Option<NodeIndex>,
    pub goto_target: HashMap<String, NodeIndex>, // shadow?
    pub global_begin: NodeIndex,
    pub global_end: NodeIndex,
    pub local_source: NodeIndex,
    pub local_sink: NodeIndex,
}

impl GraphContext {
    fn new() -> GraphContext {
        let mut graph = Graph::new();
        let begin = graph.add_node(GraphNodeType::Begin);
        let end = graph.add_node(GraphNodeType::End);
        GraphContext {
            graph,
            break_target: None,
            continue_target: None,
            goto_target: HashMap::new(),
            global_begin: begin,
            global_end: end,
            local_source: begin,
            local_sink: end,
        }
    }
}

fn build_graph(ast: &Ast, context: &mut GraphContext) -> Result<()> {
    // local_source -> [...current parsing...] -> local_sink
    let local_source = context.local_source;
    let local_sink = context.local_sink;
    let break_target = context.break_target;
    let continue_target = context.continue_target;
    match &ast.node {
        AstNode::Dummy => return Err(Error::UnexpectedDummyAstNode),
        AstNode::Compound(v) => {
            let mut sub_source = context.graph.add_node(GraphNodeType::Dummy);
            let mut sub_sink = context.graph.add_node(GraphNodeType::Dummy);
            context
                .graph
                .add_edge(local_source, sub_source, EdgeType::Normal);
            // what if v.is_empty()??
            if v.is_empty() {
                context
                    .graph
                    .add_edge(sub_source, sub_sink, EdgeType::Normal);
            } else {
                for i in v.iter().with_position() {
                    context.local_source = sub_source;
                    context.local_sink = sub_sink;
                    build_graph(&i.into_inner().borrow(), context)?;
                    match i {
                        itertools::Position::First(_) | itertools::Position::Middle(_) => {
                            sub_source = sub_sink;
                            sub_sink = context.graph.add_node(GraphNodeType::Dummy);
                        }
                        _ => {}
                    }
                }
            }
            context
                .graph
                .add_edge(sub_sink, local_sink, EdgeType::Normal);
            context.local_source = local_source;
            context.local_sink = local_sink;
        }
        AstNode::Stat(s) => {
            // local_source -> current -> local_sink
            let current = context.graph.add_node(GraphNodeType::Node(s.clone()));
            context
                .graph
                .add_edge(local_source, current, EdgeType::Normal);
            context
                .graph
                .add_edge(current, local_sink, EdgeType::Normal);
        }
        AstNode::Continue(s) => {
            // local_source -> current -> continue_target
            let current = context.graph.add_node(GraphNodeType::Node(s.clone()));
            context
                .graph
                .add_edge(local_source, current, EdgeType::Normal);
            context.graph.add_edge(
                current,
                context.continue_target.ok_or(Error::UnexpectedContinue)?,
                EdgeType::Normal,
            );
        }
        AstNode::Break(s) => {
            // local_source -> current -> break_target
            let current = context.graph.add_node(GraphNodeType::Node(s.clone()));
            context
                .graph
                .add_edge(local_source, current, EdgeType::Normal);
            context.graph.add_edge(
                current,
                context.break_target.ok_or(Error::UnexpectedBreak)?,
                EdgeType::Normal,
            );
        }
        AstNode::Return(s) => {
            // local_source -> current -> global_end
            let current = context.graph.add_node(GraphNodeType::Node(s.clone()));
            context
                .graph
                .add_edge(local_source, current, EdgeType::Normal);
            context
                .graph
                .add_edge(current, context.global_end, EdgeType::Normal);
        }
        AstNode::If {
            cond,
            body,
            otherwise,
        } => {
            // local_source -> cond -> ---Y--> sub_source -> [...body...] -> sub_sink---------------v
            //                         ---N--> sub_source1 -> Option<[...otherwise...]> -> sub_sink -> local_sink
            let cond = context.graph.add_node(GraphNodeType::Choice(cond.clone()));
            let sub_source = context.graph.add_node(GraphNodeType::Dummy);
            let sub_sink = context.graph.add_node(GraphNodeType::Dummy);
            context.graph.add_edge(local_source, cond, EdgeType::Normal);
            context
                .graph
                .add_edge(cond, sub_source, EdgeType::Branch(true));
            context
                .graph
                .add_edge(sub_sink, local_sink, EdgeType::Normal);
            context.local_source = sub_source;
            context.local_sink = sub_sink;
            // context must be restored after calling this function
            // only graph should be changed
            // so it is OK to process the other branch directly
            build_graph(&body.borrow(), context)?;
            // restore context
            context.local_source = local_source;
            context.local_sink = local_sink;

            if let Some(t) = otherwise {
                let sub_source1 = context.graph.add_node(GraphNodeType::Dummy);
                context
                    .graph
                    .add_edge(cond, sub_source1, EdgeType::Branch(false));
                context.local_source = sub_source1;
                context.local_sink = sub_sink;
                build_graph(&t.borrow(), context)?;
                context.local_source = local_source;
                context.local_sink = local_sink;
            } else {
                context
                    .graph
                    .add_edge(cond, local_sink, EdgeType::Branch(false));
            }
        }
        AstNode::While { cond, body } => {
            // local_src -> cond ---Y--> sub_source -> [...body...] -> sub_sink
            //                |  \                                         /
            //                | N \_______________________________________/
            //                v                     <<<
            //           local_sink
            // continue: jump to cond
            // break: jump to local_sink
            let cond = context.graph.add_node(GraphNodeType::Choice(cond.clone()));
            let sub_source = context.graph.add_node(GraphNodeType::Dummy);
            let sub_sink = context.graph.add_node(GraphNodeType::Dummy);
            context.graph.add_edge(local_source, cond, EdgeType::Normal);
            context
                .graph
                .add_edge(cond, sub_source, EdgeType::Branch(true));
            context
                .graph
                .add_edge(cond, local_sink, EdgeType::Branch(false));
            context.graph.add_edge(sub_sink, cond, EdgeType::Normal);
            context.continue_target = Some(cond);
            context.break_target = Some(local_sink);
            context.local_source = sub_source;
            context.local_sink = sub_sink;
            build_graph(&body.borrow(), context)?;
            context.continue_target = continue_target;
            context.break_target = break_target;
            context.local_source = local_source;
            context.local_sink = local_sink;
        }
        AstNode::DoWhile { cond, body } => {
            // local_src -> sub_source -> [...body...] -> sub_sink -> cond ---N--> local_sink
            //                    \                                    /
            //                     <-----------------Y----------------<
            // continue: jump to cond
            // break: jump to local_sink
            let sub_source = context.graph.add_node(GraphNodeType::Dummy);
            let sub_sink = context.graph.add_node(GraphNodeType::Dummy);
            let cond = context.graph.add_node(GraphNodeType::Choice(cond.clone()));
            context
                .graph
                .add_edge(local_source, sub_source, EdgeType::Normal);
            context.graph.add_edge(sub_sink, cond, EdgeType::Normal);
            context
                .graph
                .add_edge(cond, sub_source, EdgeType::Branch(true));
            context
                .graph
                .add_edge(cond, local_sink, EdgeType::Branch(false));
            context.continue_target = Some(cond);
            context.break_target = Some(local_sink);
            context.local_source = sub_source;
            context.local_sink = sub_sink;
            build_graph(&body.borrow(), context)?;
            context.continue_target = continue_target;
            context.break_target = break_target;
            context.local_source = local_source;
            context.local_sink = local_sink;
        }
        AstNode::For {
            init,
            cond,
            upd,
            body,
        } => {
            // local_source -> init -> cond ---Y--> sub_source -> [...body...] -> sub_sink -> upd
            //                           |  \                                                  /
            //                           |   \----N--> local_sink                             /
            //                           |___________________________________________________/
            //                                              <<<
            // continue: jump to sub_sink
            // break: jump to local_sink
            let sub_source = context.graph.add_node(GraphNodeType::Dummy);
            let sub_sink = context.graph.add_node(GraphNodeType::Dummy);
            let cond = context.graph.add_node(GraphNodeType::Choice(cond.clone()));
            let init = context.graph.add_node(GraphNodeType::Node(init.clone()));
            let upd = context.graph.add_node(GraphNodeType::Node(upd.clone()));
            context.graph.add_edge(local_source, init, EdgeType::Normal);
            context.graph.add_edge(init, cond, EdgeType::Normal);
            context
                .graph
                .add_edge(cond, sub_source, EdgeType::Branch(true));
            context
                .graph
                .add_edge(cond, local_sink, EdgeType::Branch(false));
            context.graph.add_edge(sub_sink, upd, EdgeType::Normal);
            context.graph.add_edge(upd, cond, EdgeType::Normal);
            context.continue_target = Some(upd);
            context.break_target = Some(local_sink);
            context.local_source = sub_source;
            context.local_sink = sub_sink;
            build_graph(&body.borrow(), context)?;
            context.continue_target = continue_target;
            context.break_target = break_target;
            context.local_source = local_source;
            context.local_sink = local_sink;
        }
        AstNode::Switch { cond, body } => todo!(),
        AstNode::Goto(_) => todo!(),
    }
    Ok(())
}

fn remove_zero_in_degree_nodes(graph: &mut Graph) -> bool {
    let nodes = graph
        .node_indices()
        .filter(|i| -> bool {
            *graph.node_weight(*i).unwrap() == GraphNodeType::Dummy
                && graph.edges_directed(*i, EdgeDirection::Incoming).count() == 0
        })
        .collect_vec();
    nodes
        .iter()
        .map(|x| graph.remove_node(*x))
        .any(|x| x.is_some())
}

// remove the first node which predicate(node) == True
// return Ok(true) if successfully remove a node
// return Ok(false) if no node is available
// return Err if there are more than one predecessors
fn remove_single_node<F>(graph: &mut Graph, predicate: F) -> Result<bool>
where
    F: Fn(NodeIndex, &GraphNodeType) -> bool,
{
    // take first dummy node
    if let Some(node_index) = graph
        .node_references()
        .filter(|(x, t)| predicate(*x, *t))
        .map(|(x, _)| x)
        .take(1)
        .next()
    {
        let incoming_edges = graph
            .edges_directed(node_index, EdgeDirection::Incoming)
            .map(|x| (x.source(), *x.weight()))
            .collect_vec();
        let neighbors = graph
            .neighbors_directed(node_index, EdgeDirection::Outgoing)
            .collect_vec();
        if neighbors.len() != 1 {
            return Err(Error::UnexpectedOutgoingNodes {
                node_index,
                neighbors,
            });
        }
        let next_node = neighbors[0];
        for (src, edge_type) in incoming_edges {
            // add edge: i.src -> next_node
            graph.add_edge(src, next_node, edge_type);
        }
        graph.remove_node(node_index);
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn from_ast(ast: Rc<RefCell<Ast>>) -> Result<Graph> {
    let mut ctx = GraphContext::new();
    build_graph(&ast.borrow(), &mut ctx)?;
    while remove_zero_in_degree_nodes(&mut ctx.graph) {}
    while remove_single_node(&mut ctx.graph, |_, t| *t == GraphNodeType::Dummy)? {}
    let remove_empty_nodes: fn(NodeIndex, &GraphNodeType) -> bool = |_, t| match t {
        GraphNodeType::Node(t) => t.is_empty(),
        _ => false,
    };
    while remove_single_node(&mut ctx.graph, remove_empty_nodes)? {}
    Ok(ctx.graph)
}
