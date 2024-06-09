use cxx2flow::{
    cli::Args,
    display::{d2::D2, dot::Dot, tikz::Tikz},
    dump,
    error::Error,
};
use std::{
    io::{Read, Write},
    process::{self, Stdio},
};
use tree_sitter::Parser;

use itertools::Itertools;

use cxx2flow::generate;
use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    miette::set_panic_hook();
    let args = <Args as clap::Parser>::parse();
    let mut content: Vec<u8> = Vec::new();
    match args.input {
        Some(ref file_name) => {
            content = std::fs::read(file_name).into_diagnostic()?;
        }
        None => {
            std::io::stdin()
                .read_to_end(&mut content)
                .into_diagnostic()?;
        }
    };
    let content = if args.cpp {
        let mut cpp = process::Command::new("cpp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .into_diagnostic()?;
        if let Some(mut child_stdin) = cpp.stdin.take() {
            child_stdin.write_all(&content).into_diagnostic()?;
        }
        cpp.wait_with_output().into_diagnostic()?.stdout
    } else {
        content
    };
    let content = Itertools::intersperse(
        String::from_utf8(content)
            .unwrap()
            .lines()
            .filter(|x| !x.starts_with('#')),
        "\n",
    )
    .collect::<String>()
    .into_bytes();
    if args.dump_ast {
        let mut parser = Parser::new();
        let language = tree_sitter_cpp::language();
        parser
            .set_language(&language)
            .map_err(|_| Error::TreesitterParseFailed)?;
        let tree = parser
            .parse(&content, None)
            .ok_or(Error::TreesitterParseFailed)?;
        dump::dump_node(&tree.root_node(), &content);
        return Ok(());
    }
    let backend = if args.tikz {
        Tikz::new().into()
    } else if args.d2 {
        D2::new().into()
    } else {
        Dot::new(args.curly).into()
    };
    let res = generate(
        &content,
        &args.input.unwrap_or_else(|| "stdin".to_owned()),
        Some(args.function),
        backend,
    )?;
    if let Some(output) = args.output {
        std::fs::write(output, res).into_diagnostic()?;
    } else {
        print!("{}", res);
    }
    Ok(())
}
