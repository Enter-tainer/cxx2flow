use clap::Parser;
use cxx2flow::{error::Result, generate};

#[derive(Parser, Debug)]
#[clap(about, version, author, after_help("Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.

EXAMPLES:
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg"))]
struct Args {
    #[clap(
        short,
        long,
        help(
            "Sets the output file.
If not specified, result will be directed to stdout.
e.g. graph.dot"
        )
    )]
    output: Option<String>,

    #[clap(
        short,
        long,
        help(
            "Sets the style of the flow chart.
If specified, output flow chart will have curly connection line."
        )
    )]
    curly: bool,

    #[clap(short, long, help("Use tikz backend."))]
    tikz: bool,

    #[clap(required(true), help("Sets the input file. e.g. test.cpp"))]
    input: String,

    #[clap(
        default_value("main"),
        help("The function you want to convert. e.g. main")
    )]
    function: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let content = std::fs::read(args.input)?;
    let res = generate(&content, Some(args.function), args.curly, args.tikz)?;
    if let Some(output) = args.output {
        std::fs::write(output, res)?;
    } else {
        print!("{}", res);
    }
    Ok(())
}
