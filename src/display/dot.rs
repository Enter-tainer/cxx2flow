use crate::error::{Error, Result};
use crate::graph::{Graph, GraphNodeType};
use petgraph::{
    visit::IntoNodeReferences,
    visit::{EdgeRef, IntoEdgeReferences},
};

use super::GraphDisplay;

pub struct Dot {
    curly: bool,
}

impl Dot {
    pub fn new(curly: bool) -> Self {
        Dot { curly }
    }
}

impl GraphDisplay for Dot {
    fn generate_from_graph(&self, graph: &Graph) -> Result<String> {
        let mut res = "digraph {\n".to_string();
        if !self.curly {
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
                        str.replace('\"', "\\\"")
                    )
                    .as_str(),
                ),
                GraphNodeType::Choice(str) => res.push_str(
                    format!(
                        "D{} [shape=diamond, label=\"{}?\"];\n",
                        id.index(),
                        str.replace('\"', "\\\"")
                    )
                    .as_str(),
                ),
                GraphNodeType::Dummy => {
                    return Err(Error::UnexpectedDummyGraphNode {
                        graph: graph.clone(),
                    })
                } // GraphNodeType::Dummy => {} // all dummy node will be eliminated
            }
        }

        for i in graph.edge_references() {
            match i.weight() {
                crate::graph::EdgeType::Normal => res.push_str(
                    format!("D{} -> D{};\n", i.source().index(), i.target().index()).as_str(),
                ),
                crate::graph::EdgeType::Branch(t) => res.push_str(
                    format!(
                        "D{}:{} -> D{}:n [xlabel={}];\n",
                        i.source().index(),
                        if *t { "s" } else { "e" },
                        i.target().index(),
                        if *t { "Y" } else { "N" }
                    )
                    .as_str(),
                ),
            };
        }
        res.push_str("}\n");
        Ok(res)
    }
}
