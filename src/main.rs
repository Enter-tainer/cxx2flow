#[macro_use]
extern crate clap;
mod ast;
mod dot;
mod graph;
mod parser;

fn main() -> anyhow::Result<()> {
    let matches = clap_app!(cxx2flow =>
        (version: "0.1.0")
        (author: "mgt. <mgt@oi-wiki.org>")
        (about: "Convert your C/C++ code to control flow graph")
        (@arg OUTPUT: -o --output +takes_value "Sets the output file. e.g. graph.dot")
        (@arg INPUT: +required "Sets the input file. e.g. test.cpp")
        (@arg FUNCTION: "The function you want to convert. e.g. main")
    )
    .setting(clap::AppSettings::ColoredHelp)
    .get_matches();
    let path = matches.value_of("INPUT").unwrap();
    let func = matches.value_of("FUNCTION").map(|x| x.to_string());
    let output = matches.value_of("OUTPUT");
    let ast_vec = parser::parse(path, func)?;
    let graph = graph::from_ast(ast_vec)?;
    let dot = dot::from_graph(&graph)?;
    if let Some(output) = output {
        std::fs::write(output, dot)?;
    } else {
        print!("{}", dot);
    }
    Ok(())
}
