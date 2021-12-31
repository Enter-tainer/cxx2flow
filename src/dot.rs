use crate::error::{Error, Result};
use crate::graph::{Graph, GraphNodeType};
use petgraph::{
    visit::IntoNodeReferences,
    visit::{EdgeRef, IntoEdgeReferences},
};
pub fn from_graph(graph: &Graph, curved: bool) -> Result<String> {
    let mut res = "digraph {\n".to_string();
    if !curved {
        res.push_str("graph [splines=polyline];\n");
    }
    for (id, i) in graph.node_references() {
        match i {
            GraphNodeType::Begin => res.push_str(
                format!(
                    "D{} [shape=box, style=rounded, label=\"begin\"];\n",
                    id.index()
                )
                .as_str(),
            ),
            GraphNodeType::End => res.push_str(
                format!(
                    "{{rank = sink; D{} [shape=box, style=rounded, label=\"end\"];}}\n",
                    id.index()
                )
                .as_str(),
            ),
            GraphNodeType::Node(str) => res.push_str(
                format!(
                    "D{} [shape=box, label=\"{}\"];\n",
                    id.index(),
                    str.replace("\"", "\\\"")
                )
                .as_str(),
            ),
            GraphNodeType::Choice(str) => res.push_str(
                format!(
                    "D{} [shape=diamond, label=\"{}?\"];\n",
                    id.index(),
                    str.replace("\"", "\\\"")
                )
                .as_str(),
            ),
            // GraphNodeType::Dummy => return Err(Error::UnexpectedDummyGraphNode),
            GraphNodeType::Dummy => {}
            // all dummy node will be eliminated
        }
    }

    for i in graph.edge_references() {
        match i.weight() {
            crate::graph::EdgeType::Normal => res.push_str(
                format!("D{}:s -> D{}:n;\n", i.source().index(), i.target().index()).as_str(),
            ),
            crate::graph::EdgeType::Branch(t) => res.push_str(
                format!(
                    "D{}:s -> D{}:n [xlabel={}];\n",
                    i.source().index(),
                    i.target().index(),
                    if *t { "Y" } else { "N" }
                )
                .as_str(),
            ),
        };
    }
    res.push('}');
    Ok(res)
}
