use std::env::args;

use parser::parse;

mod ast;
mod dot;
mod graph;
mod parser;
fn main() -> anyhow::Result<()> {
    let path = "test.cpp";
    let ast_vec = parse(path, args().nth(1))?;
    let graph = graph::from_ast(ast_vec)?;
    // dbg!(graph);
    let dot = dot::from_graph(&graph)?;
    print!("{}", dot);
    Ok(())
}
