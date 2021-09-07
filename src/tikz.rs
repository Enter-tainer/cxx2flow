use crate::{
    graph::{Graph, GraphNodeType},
};
use anyhow::Result;

pub fn from_graph(graph: &Graph, curved: bool) -> Result<String> {
    let mut res = r#"
\documentclass[tikz,border=10pt]{standalone}
\usepackage{ctex}
\usetikzlibrary{graphdrawing}
\usetikzlibrary{shapes}
\usepackage{verbatim}
\usetikzlibrary{graphs}
\usegdlibrary{layered}
\begin{document}
\tikz [layered layout] {
  "#
    .to_string();
    for i in graph {
        match i.node_type {
            GraphNodeType::Start => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rounded rectangle] {{ \\verb|{}| }};\n",
                    i.id, i.content.replace('%', "\\%")
                )
                .as_str(),
            ),
            GraphNodeType::End => res.push_str(
                format!(
                    "\\node[draw] (D{}) [rounded rectangle] {{ \\verb|{}| }};\n",
                    i.id, i.content.replace('%', "\\%")
                )
                .as_str(),
            ),
            GraphNodeType::Node(_) => res.push_str(
                format!("\\node[draw] (D{}) [rectangle] {{ \\verb|{}| }};\n", i.id, i.content.replace('%', "\\%")).as_str(),
            ),
            GraphNodeType::Choice(_, _) => res
                .push_str(format!("\\node[draw] (D{}) [diamond, aspect=5] {{ \\verb|{}| }};\n", i.id, i.content.replace('%', "\\%")).as_str()),
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
                    res.push_str(format!("\\draw (D{}) edge[->] (D{});\n", i.id, id).as_str())
                }
                None => {
                    res.push_str(format!("\\draw (D{}) edge[->] (D{});\n", i.id, i.id + 1).as_str())
                }
            },
            GraphNodeType::Choice(t, f) => {
                res.push_str(
                    format!(
                        "\\draw (D{}) edge[->] node {{ Y }} (D{});\n",
                        i.id,
                        t.unwrap_or(i.id + 1)
                    )
                    .as_str(),
                );
                res.push_str(
                    format!(
                        "\\draw (D{}) edge[->] node {{ N }} (D{});\n",
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
