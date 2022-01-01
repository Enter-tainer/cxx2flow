use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};

use crate::error::{Error, Result};
use crate::graph::{Graph, GraphNodeType};

pub fn from_graph(graph: &Graph, _curved: bool) -> Result<String> {
    let mut res = r#"
\documentclass[tikz,border=10pt]{standalone}
\usepackage{ctex}
\usetikzlibrary{graphdrawing}
\usetikzlibrary{shapes}
\usepackage{spverbatim}
\usepackage{varwidth}
\usetikzlibrary{graphs}
\usegdlibrary{layered}
\usepackage[T1]{fontenc}% NOT OT1!
\usepackage{lmodern}% Latin Modern fonts,
  % a modern variant of Computer Modern fonts
\let\ttdefault\rmdefault
\tikzstyle{block} = [%
   draw,thick,fill=blue!0,
   inner sep=0.3cm,
   text centered, minimum height=1em,
   execute at begin node={\begin{varwidth}{8em}},
   execute at end node={\end{varwidth}}]
\begin{document}
\tikz [layered layout, sibling distance=3cm] {
  "#
    .to_string();
    for (id, i) in graph.node_references() {
        match i {
            GraphNodeType::Begin => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rounded rectangle, block] {{ Begin }};\n",
                    id.index()
                )
                .as_str(),
            ),
            GraphNodeType::End => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rounded rectangle, block] {{ End }};\n",
                    id.index()
                )
                .as_str(),
            ),
            GraphNodeType::Node(str) => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rectangle, block] {{ \\spverb${}$ }};\n",
                    id.index(),
                    str.replace('%', "\\%")
                )
                .replace('\n', " ")
                .as_str(),
            ),
            GraphNodeType::Choice(str) => res.push_str(
                format!(
                    "\\node[draw] (D{}) [diamond, aspect=2, block] {{ \\spverb${}$ }};\n",
                    id.index(),
                    str.replace('%', "\\%")
                )
                .replace('\n', " ")
                .as_str(),
            ),
            GraphNodeType::Dummy => return Err(Error::UnexpectedDummyGraphNode),
            // all dummy node will be eliminated
        }
    }
    for i in graph.edge_references() {
        match i.weight() {
            crate::graph::EdgeType::Normal => res.push_str(
                format!(
                    "\\draw (D{}) edge[->] (D{});\n",
                    i.source().index(),
                    i.target().index()
                )
                .as_str(),
            ),
            crate::graph::EdgeType::Branch(t) => res.push_str(
                format!(
                    "\\draw (D{}) edge[->, below] node {{ {} }} (D{});\n",
                    i.source().index(),
                    i.target().index(),
                    if *t { "Y" } else { "N" }
                )
                .as_str(),
            ),
        }
    }
    res.push_str(
        r#"
}
\end{document}
  "#,
    );
    Ok(res)
}
