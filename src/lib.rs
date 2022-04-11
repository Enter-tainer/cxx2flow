mod ast;
pub mod cli;
mod display;
mod dump;
pub mod error;
mod graph;
mod parser;
use display::{dot::Dot, tikz::Tikz, GraphDisplay, GraphDisplayBackend};
use error::Result;
pub fn generate(
    content: &[u8],
    file_name: &str,
    function_name: Option<String>,
    curly: bool,
    tikz: bool,
) -> Result<String> {
    let ast = parser::parse(content, file_name, function_name)?;
    dbg!(&ast);
    let graph = graph::from_ast(ast, &String::from_utf8(content.to_vec())?, file_name)?;
    // dbg!(&graph);
    let display: GraphDisplayBackend = if tikz {
        Tikz::new().into()
    } else {
        Dot::new(curly).into()
    };
    display.from_graph(&graph)
}
