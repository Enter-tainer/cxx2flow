mod ast;
mod dot;
mod graph;
mod parser;
mod tikz;
mod dump;
mod error;
use crate::error::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author, after_help("Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.

EXAMPLES:
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg"))]
struct Args {
    #[clap(short, long, help("Sets the output file.
If not specified, result will be directed to stdout.
e.g. graph.dot"))]
    output: Option<String>,
    
    #[clap(short, long, help("Sets the style of the flow chart.
If specified, output flow chart will have curly connection line."))]
    curly: bool,
    
    #[clap(short, long, help("Use tikz backend."))]
    tikz: bool,
    
    #[clap(required(true), help("Sets the input file. e.g. test.cpp"))]
    input: String,
    
    #[clap(default_value("main"), help("The function you want to convert. e.g. main"))]
    function: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (ast, maxid) = parser::parse(&args.input, Some(args.function))?;
    dbg!(&ast);
    let graph = graph::from_ast(ast)?;
    dbg!(&graph);
    let res = if args.tikz {
        tikz::from_graph(&graph, args.curly)
    } else {
        dot::from_graph(&graph, args.curly)
    }?;
    if let Some(output) = args.output {
        std::fs::write(output, res)?;
    } else {
        print!("{}", res);
    }
    Ok(())
}
