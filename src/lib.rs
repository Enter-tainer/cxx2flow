mod ast;
pub mod cli;
pub mod display;
pub mod dump;
pub mod error;
mod graph;
mod parser;
use display::{GraphDisplay, GraphDisplayBackend};
use error::Result;
pub fn generate(
    content: &[u8],
    file_name: &str,
    function_name: Option<String>,
    backend: GraphDisplayBackend,
) -> Result<String> {
    let ast = parser::parse(content, file_name, function_name)?;
    // dbg!(&ast);
    let graph = graph::from_ast(ast, &String::from_utf8(content.to_vec())?, file_name)?;
    // dbg!(&graph);
    backend.generate_from_graph(&graph)
}
