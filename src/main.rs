extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use parser::parse;

mod ast;
mod graph;
mod parser;
fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let path = "test.cpp";
    let ast_vec = parse(path)?;
    info!("{:#?}", ast_vec);
    let graph = graph::from_ast(ast_vec)?;
    dbg!(graph);
    Ok(())
}
