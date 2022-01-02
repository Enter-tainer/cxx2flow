
use cxx2flow::generate;
use once_cell::sync::Lazy;

use clap::Parser;
use miette::IntoDiagnostic;

#[derive(Parser, Debug)]
#[clap(about, version, long_version(get_long_version_string()) ,author, after_help("Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.

EXAMPLES:
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg

Please give me star if this application helps you!
如果这个应用有帮助到你，请给我点一个 star！
https://github.com/Enter-tainer/cxx2flow
"))]
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

fn get_long_version_string() -> &'static str {
    static CELL: Lazy<String> = Lazy::new(|| {
        format!(
            "
Build Timestamp:     {}
Build Version:       {}
Commit SHA:          {:?}
Commit Date:         {:?}
Commit Branch:       {:?}
Cargo Target Triple: {}
Cargo Profile:       {}
",
            env!("VERGEN_BUILD_TIMESTAMP"),
            env!("VERGEN_BUILD_SEMVER"),
            option_env!("VERGEN_GIT_SHA"),
            option_env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
            option_env!("VERGEN_GIT_BRANCH"),
            env!("VERGEN_CARGO_TARGET_TRIPLE"),
            env!("VERGEN_CARGO_PROFILE")
        )
    });
    CELL.as_str()
}

fn main() -> miette::Result<()> {
    miette::set_panic_hook();
    let args = Args::parse();
    let content = std::fs::read(&args.input).into_diagnostic()?;
    let res = futures::executor::block_on(generate(
        &content,
        &args.input,
        Some(args.function),
        args.curly,
        args.tikz,
    ))?;
    if let Some(output) = args.output {
        std::fs::write(output, res).into_diagnostic()?;
    } else {
        print!("{}", res);
    }
    Ok(())
}
