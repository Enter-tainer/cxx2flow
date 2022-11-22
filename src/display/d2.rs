use crate::error::{Error, Result};
use crate::graph::{Graph, GraphNodeType};
use petgraph::{
    visit::IntoNodeReferences,
    visit::{EdgeRef, IntoEdgeReferences},
};

use super::GraphDisplay;
#[derive(Debug, Default)]
pub struct D2 {}

impl D2 {
    pub fn new() -> Self {
        D2 {}
    }
}

impl GraphDisplay for D2 {
    fn generate_from_graph(&self, graph: &Graph) -> Result<String> {
        let mut res = String::new();
        for (id, i) in graph.node_references() {
            match i {
                GraphNodeType::Begin => res.push_str(format!("D{}: begin\n", id.index()).as_str()),
                GraphNodeType::End => res.push_str(format!("D{}: end\n", id.index()).as_str()),
                GraphNodeType::Node(str) => res.push_str(
                    format!(
                        "D{}: \"{}\"\n",
                        id.index(),
                        str.replace('\"', "\\\"").replace('\n', "\\n")
                    )
                    .as_str(),
                ),
                GraphNodeType::Choice(str) => {
                    res.push_str(
                        format!("D{}: \"{}\"\n", id.index(), str.replace('\"', "\\\"")).as_str(),
                    );
                    res.push_str(format!("D{}.shape: diamond\n", id.index()).as_str());
                }
                GraphNodeType::Dummy => {
                    return Err(Error::UnexpectedDummyGraphNode {
                        graph: graph.clone(),
                    })
                }
            }
        }
        for i in graph.edge_references() {
            match i.weight() {
                crate::graph::EdgeType::Normal => res.push_str(
                    format!("D{} -> D{}\n", i.source().index(), i.target().index()).as_str(),
                ),
                crate::graph::EdgeType::Branch(t) => res.push_str(
                    format!(
                        "D{} -> D{}: {}\n",
                        i.source().index(),
                        i.target().index(),
                        if *t { "Y" } else { "N" }
                    )
                    .as_str(),
                ),
            };
        }
        Ok(res)
    }
}
