use crate::graph::{Graph, GraphNodeType};
use anyhow::Result;

pub fn from_graph(graph: &Graph, curved: bool) -> Result<String> {
    let mut res = "digraph { \n concentrate=true; \n".to_string();
    if !curved {
        res.push_str("graph [splines=polyline];\n");
    }
    for i in graph {
        match i.node_type {
            GraphNodeType::Start => res.push_str(
                format!(
                    "D{} [shape=box, style=rounded, label=\"{}\"];\n",
                    i.id, i.content
                )
                .as_str(),
            ),
            GraphNodeType::End => res.push_str(
                format!(
                    "{{rank = sink; D{} [shape=box, style=rounded, label=\"{}\"];}}\n",
                    i.id, i.content
                )
                .as_str(),
            ),
            GraphNodeType::Node(_) => res.push_str(
                format!(
                    "D{} [shape=box, label=\"{}\"];\n",
                    i.id,
                    i.content.replace("\"", "\\\"")
                )
                .as_str(),
            ),
            GraphNodeType::Choice(_, _) => res.push_str(
                format!(
                    "D{} [shape=diamond, label=\"{}?\"];\n",
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
                res.push_str(format!("D{}:s -> D{}:n;\n", i.id, i.id + 1).as_str());
            }
            GraphNodeType::End => res.push_str("}\n"),
            GraphNodeType::Node(t) => match t {
                Some(id) => res.push_str(format!("D{} -> D{};\n", i.id, id).as_str()),
                None => res.push_str(format!("D{}:s-> D{}:n;\n", i.id, i.id + 1).as_str()),
            },
            GraphNodeType::Choice(t, f) => {
                res.push_str(
                    format!("D{}:s -> D{}:n [xlabel=Y];\n", i.id, t.unwrap_or(i.id + 1)).as_str(),
                );
                res.push_str(format!("D{}:e -> D{}:n [xlabel=N];\n", i.id, f.unwrap()).as_str());
            }
        }
    }
    Ok(res)
}
