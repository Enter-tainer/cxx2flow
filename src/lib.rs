mod ast;
mod dot;
mod dump;
pub mod error;
mod graph;
mod parser;
mod tikz;
use error::Result;
pub fn generate(
    content: &[u8],
    function_name: Option<String>,
    curly: bool,
    tikz: bool,
) -> Result<String> {
    let ast = parser::parse(content, function_name)?;
    // dbg!(&ast);
    let graph = graph::from_ast(ast)?;
    // dbg!(&graph);
    if tikz {
        tikz::from_graph(&graph, curly)
    } else {
        dot::from_graph(&graph, curly)
    }
}
