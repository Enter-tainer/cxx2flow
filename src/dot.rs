use crate::graph::{Graph, GraphNodeType};
use anyhow::Result;

pub fn from_graph(graph: &Graph, curved: bool) -> Result<String> {
    let mut res = "digraph { \n".to_string();
    if !curved {
        res.push_str("graph [splines=ortho, nodesep=1];\n");
    }
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
            GraphNodeType::End => res.push_str("}\n"),
            GraphNodeType::Node => match i.jump {
                Some(id) => res.push_str(format!("D{} -> D{};\n", i.id, id).as_str()),
                None => res.push_str(format!("D{} -> D{};\n", i.id, i.id + 1).as_str()),
            },
            GraphNodeType::Choice => {
                res.push_str(format!("D{} -> D{} [xlabel=\"Y\"];\n", i.id, i.id + 1).as_str());
                res.push_str(
                    format!("D{} -> D{} [xlabel=\"N\"];\n", i.id, i.jump.unwrap()).as_str(),
                );
            }
        }
    }
    Ok(res)
}
