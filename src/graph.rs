use crate::ast::Ast;
use anyhow::Result;
#[derive(Debug)]
pub enum GraphNodeType {
    Start,
    End,
    Node,
    Choice,
}

#[derive(Debug)]
pub struct GraphNode {
    pub id: usize,
    pub node_type: GraphNodeType,
    pub content: String,
    pub jump: Option<usize>,
}

pub type Graph = Vec<GraphNode>;

fn transform_ast_impl(
    node: &Ast,
    continue_vec: &mut Vec<usize>,
    break_vec: &mut Vec<usize>,
    return_vec: &mut Vec<usize>,
    graph: &mut Graph,
) -> Result<()> {
    match &node {
        Ast::Stat(content)
        | Ast::Return(content)
        | Ast::Continue(content)
        | Ast::Break(content) => {
            let graph_node: GraphNode = GraphNode {
                id: graph.len(),
                node_type: GraphNodeType::Node,
                content: content.clone(),
                jump: None,
            };
            if let Ast::Break(_) = &node {
                break_vec.push(graph_node.id);
            }
            if let Ast::Continue(_) = &node {
                continue_vec.push(graph_node.id);
            }
            if let Ast::Return(_) = &node {
                return_vec.push(graph_node.id);
            }
            graph.push(graph_node);
        }
        Ast::If(cond, chld1, chld2) => {
            let choice_node = GraphNode {
                id: graph.len(),
                node_type: GraphNodeType::Choice,
                content: cond.clone(),
                jump: None,
            };
            let choice_id = graph.len();
            graph.push(choice_node);
            for i in chld1 {
                transform_ast_impl(i, continue_vec, break_vec, return_vec, graph)?;
            }
            let block1_last_id = graph.len() - 1;
            for i in chld2 {
                transform_ast_impl(i, continue_vec, break_vec, return_vec, graph)?;
            }
            let block2_last_id = graph.len() - 1;
            graph[choice_id].jump = Some(block1_last_id + 1);
            graph[block1_last_id].jump = Some(block2_last_id + 1);
        }
        Ast::While(cond, chld) => {
            let choice_node = GraphNode {
                id: graph.len(),
                node_type: GraphNodeType::Choice,
                content: cond.clone(),
                jump: None,
            };
            let choice_id = graph.len();
            graph.push(choice_node);
            let mut continue_vec_inner: Vec<usize> = Vec::new();
            let mut break_vec_inner: Vec<usize> = Vec::new();
            for i in chld {
                transform_ast_impl(
                    i,
                    &mut continue_vec_inner,
                    &mut break_vec_inner,
                    return_vec,
                    graph,
                )?;
            }
            let block_last_id = graph.len() - 1;
            // what if block is empty?
            graph[block_last_id].jump = Some(choice_id);
            graph[choice_id].jump = Some(block_last_id + 1);
            for i in break_vec_inner {
                graph[i].jump = Some(block_last_id + 1);
            }
            for i in continue_vec_inner {
                graph[i].jump = Some(choice_id);
            }
        }
        Ast::For(init, cond, upd, chld) => {
            if !init.is_empty() {
                let init_node = GraphNode {
                    id: graph.len(),
                    node_type: GraphNodeType::Node,
                    content: init.clone(),
                    jump: None,
                };
                graph.push(init_node);
            }
            let choice_node = GraphNode {
                id: graph.len(),
                node_type: GraphNodeType::Choice,
                content: cond.clone(),
                jump: None,
            };
            let choice_id = graph.len();
            graph.push(choice_node);
            let mut continue_vec_inner: Vec<usize> = Vec::new();
            let mut break_vec_inner: Vec<usize> = Vec::new();
            for i in chld {
                transform_ast_impl(
                    i,
                    &mut continue_vec_inner,
                    &mut break_vec_inner,
                    return_vec,
                    graph,
                )?;
            }
            if !upd.is_empty() {
                let upd_node = GraphNode {
                    id: graph.len(),
                    node_type: GraphNodeType::Node,
                    content: upd.clone(),
                    jump: Some(choice_id),
                };
                graph.push(upd_node);
            }
            let last_id = graph.len() - 1;
            graph[last_id].jump = Some(choice_id);
            graph[choice_id].jump = Some(last_id + 1);
            for i in continue_vec_inner {
                graph[i].jump = Some(last_id);
            }
            for i in break_vec_inner {
                graph[i].jump = Some(last_id + 1);
            }
        }
    }
    Ok(())
}

fn transform_ast(node: &Ast, return_vec: &mut Vec<usize>, graph: &mut Graph) -> Result<()> {
    let mut continue_vec: Vec<usize> = Vec::new();
    let mut break_vec: Vec<usize> = Vec::new();
    transform_ast_impl(node, &mut continue_vec, &mut break_vec, return_vec, graph)
}

pub fn from_ast(ast: Vec<Ast>) -> Result<Graph> {
    let mut graph: Graph = Vec::new();
    graph.push(GraphNode {
        id: 0,
        node_type: GraphNodeType::Start,
        content: "Start".to_string(),
        jump: None,
    });
    let mut return_vec: Vec<usize> = Vec::new();
    for i in ast {
        transform_ast(&i, &mut return_vec, &mut graph)?;
    }
    for i in return_vec {
        graph[i].jump = Some(graph.len());
    }
    graph.push(GraphNode {
        id: graph.len(),
        node_type: GraphNodeType::End,
        content: "End".to_string(),
        jump: None,
    });
    Ok(graph)
}
