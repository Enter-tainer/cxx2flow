use crate::{graph::{GraphNode, GraphNodeType}};
use anyhow::Result;

pub fn from_graph(graph: &[GraphNode], _curved: bool) -> Result<String> {
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
    for i in graph {
        match i.node_type {
            GraphNodeType::Start => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rounded rectangle, block] {{ \\spverb${}$ }};\n",
                    i.id, i.content.replace('%', "\\%")
                )
                .as_str(),
            ),
            GraphNodeType::End => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rounded rectangle, block] {{ \\spverb${}$ }};\n",
                    i.id, i.content.replace('%', "\\%")
                )
                .as_str(),
            ),
            GraphNodeType::Node(_) => res.push_str(
                format!("\\node[draw] (D{}) [rectangle, block] {{ \\spverb${}$ }};\n", i.id, i.content.replace('%', "\\%")).replace('\n', " ").as_str(),
            ),
            GraphNodeType::Choice(_, _) => res
                .push_str(format!("\\node[draw] (D{}) [diamond, aspect=2, block] {{ \\spverb${}$ }};\n", i.id, i.content.replace('%', "\\%")).replace('\n', " ").as_str()),
        }
    }
    for i in graph {
        match i.node_type {
            GraphNodeType::Start => {
                res.push_str(format!("\\draw (D{}) edge[->] (D{});\n", i.id, i.id + 1).as_str())
            }
            GraphNodeType::End => {
                res.push_str(
                    r#"
}
\end{document}
  "#,
                );
            }
            GraphNodeType::Node(t) => match t {
                Some(id) => {
                    res.push_str(format!("\\draw (D{}) edge[->, below] (D{});\n", i.id, id).as_str())
                }
                None => {
                    res.push_str(format!("\\draw (D{}) edge[->] (D{});\n", i.id, i.id + 1).as_str())
                }
            },
            GraphNodeType::Choice(t, f) => {
                res.push_str(
                    format!(
                        "\\draw (D{}) edge[->, below] node {{ Y }} (D{});\n",
                        i.id,
                        t.unwrap_or(i.id + 1)
                    )
                    .as_str(),
                );
                res.push_str(
                    format!(
                        "\\draw (D{}) edge[->, below] node {{ N }} (D{});\n",
                        i.id,
                        f.unwrap()
                    )
                    .as_str(),
                );
            }
        }
    }
    // \draw (a) edge[->] (b);

    Ok(res)
}
