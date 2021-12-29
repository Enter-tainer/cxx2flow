#[macro_use]
extern crate clap;
mod ast;
// mod dot;
// mod graph;
mod parser;
// mod tikz;
mod dump;
mod error;
use crate::error::Result;

fn main() -> Result<()> {
    let matches = clap_app!(cxx2flow =>
        (version: "0.3.0")
        (author: "mgt. <mgt@oi-wiki.org>")
        (about: "Convert your C/C++ code to control flow chart")
        (@arg OUTPUT: -o --output +takes_value "Sets the output file.
If not specified, result will be directed to stdout.
e.g. graph.dot")
        (@arg curved: -c --curved "Sets the style of the flow chart.
If specified, output flow chart will have curved connection line.")
        (@arg tikz: -t --tikz "Use tikz backend.")
        (@arg INPUT: +required "Sets the input file. e.g. test.cpp")
        (@arg FUNCTION: "The function you want to convert. e.g. main")
    )
    .after_help("Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.
EXAMPLES:
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg")
    .setting(clap::AppSettings::ColoredHelp)
    .get_matches();
    let path = matches.value_of("INPUT").unwrap();
    let func = matches.value_of("FUNCTION").map(|x| x.to_string());
    let output = matches.value_of("OUTPUT");
    let curved = matches.is_present("curved");
    let tikz = matches.is_present("tikz");
    let (ast_vec, maxid) = parser::parse(path, func)?;
    dbg!(&ast_vec);
    // let graph = graph::from_ast(ast_vec, maxid)?;
    // dbg!(&graph);
    // let res = if tikz {
    //     tikz::from_graph(&graph, curved)
    // } else {
    //     dot::from_graph(&graph, curved)
    // }?;
    // if let Some(output) = output {
        // std::fs::write(output, res)?;
    // } else {
        // print!("{}", res);
    // }
    Ok(())
}
