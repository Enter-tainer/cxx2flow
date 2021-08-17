use crate::graph::{Graph, GraphNodeType};
use anyhow::Result;

pub fn from_graph(graph: &Graph) -> Result<String> {
    let mut res = "digraph { \n".to_string();
    for i in graph {
        match i.node_type {
            GraphNodeType::Start | GraphNodeType::End => res.push_str(
                format!(
                    "D{} [shape=box, style=rounded, label=\"{}\"];\n",
                    i.id, i.content
                )
                .as_str(),
            ),
            GraphNodeType::Node => res.push_str(
                format!(
                    "D{} [shape=box, label=\"{}\"];\n",
                    i.id,
                    i.content.replace("\"", "\\\"")
                )
                .as_str(),
            ),
            GraphNodeType::Choice => res.push_str(
                format!(
                    "D{} [shape=diamond, label=\"{}\"];\n",
                    i.id,
                    i.content.replace("\"", "\\\"")
                )
                .as_str(),
            ),
        }
    }
    for i in graph {
        match i.node_type {
            GraphNodeType::Start => {
                res.push_str(format!("D{} -> D{};\n", i.id, i.id + 1).as_str());
            }
            GraphNodeType::End => res.push_str("} "),
            GraphNodeType::Node => match i.jump {
                Some(id) => res.push_str(format!("D{} -> D{};\n", i.id, id).as_str()),
                None => res.push_str(format!("D{} -> D{};\n", i.id, i.id + 1).as_str()),
            },
            GraphNodeType::Choice => {
                res.push_str(format!("D{} -> D{} [label=\"Y\"];\n", i.id, i.id + 1).as_str());
                res.push_str(
                    format!("D{} -> D{} [label=\"N\"];\n", i.id, i.jump.unwrap()).as_str(),
                );
            }
        }
    }
    Ok(res)
}
