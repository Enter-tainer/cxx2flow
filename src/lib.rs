mod ast;
mod dot;
mod dump;
pub mod error;
mod graph;
mod parser;
mod tikz;
mod language;
use error::Result;
use language::get_lang;
pub async fn generate(
    content: &[u8],
    file_name: &str,
    function_name: Option<String>,
    curly: bool,
    tikz: bool,
) -> Result<String> {
    let ast = parser::parse(content, file_name, get_lang().await, function_name)?;
    // dbg!(&ast);
    let graph = graph::from_ast(ast, &String::from_utf8(content.to_vec())?, file_name)?;
    // dbg!(&graph);
    if tikz {
        tikz::from_graph(&graph, curly)
    } else {
        dot::from_graph(&graph, curly)
    }
}
