use parser::parse;

mod ast;
mod counter;
mod graph;
mod parser;
fn main() {
    let path = "test.cpp";
    parse(path);
}
