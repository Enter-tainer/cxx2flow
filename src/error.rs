use miette::{Diagnostic, NamedSource, SourceSpan};
use petgraph::graph::NodeIndex;
use thiserror::Error;

use crate::graph::Graph;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("transparent")]
    #[diagnostic(
        code(cxx2flow::tree_sitter),
        help("error with tree_sitter parsing library")
    )]
    TreeSitter(#[from] tree_sitter::LanguageError),

    #[error("transparent")]
    #[diagnostic(code(cxx2flow::io), help("error with reading/writing file"))]
    Io(#[from] std::io::Error),

    #[error("transparent")]
    #[diagnostic(code(cxx2flow::utf8), help("error with UTF-8 decoding"))]
    UTF8(#[from] std::str::Utf8Error),

    #[error("transparent")]
    #[diagnostic(code(cxx2flow::from_utf8), help("error with UTF-8 decoding"))]
    FromUTF8(#[from] std::string::FromUtf8Error),

    #[error("target function not found")]
    #[diagnostic(
        code(cxx2flow::target_function_not_found),
        help("maybe you have a typo, or source code is incomplete, \nplease check your input")
    )]
    FunctionNotFound {
        #[source_code]
        src: String,
        #[label("this is the name of your target function")]
        range: SourceSpan,
    },

    #[error("declarator not found")]
    #[diagnostic(
        code(cxx2flow::declarator_not_found),
        help("maybe source code is incomplete, \nplease check your input")
    )]
    DeclaratorNotFound,

    #[diagnostic(
        code(cxx2flow::garbage_token),
        help("garbage token found in AST\nthis might be a bug, please report it to the author")
    )]
    #[error("garbage token {0}")]
    GarbageToken(&'static str),

    #[diagnostic(
        code(cxx2flow::unexpected_continue),
        help("maybe you have a continue in a wrong place(e.g. out of a loop)")
    )]
    #[error("unexpected continue")]
    UnexpectedContinue {
        #[source_code]
        src: NamedSource,
        #[label("unexpected continue statement here")]
        range: SourceSpan,
    },

    #[diagnostic(
        code(cxx2flow::unexpected_break),
        help("maybe you have a break in a wrong place(e.g. out of a loop/switch)")
    )]
    #[error("unexpected break")]
    UnexpectedBreak {
        #[source_code]
        src: NamedSource,
        #[label("unexpected break statement here")]
        range: SourceSpan,
    },

    #[diagnostic(code(cxx2flow::unexpected_dummy_graph), help("dummy node found in the flow graph\nthis might be a bug, please report it to the author"))]
    #[error("unexpected dummy graph node {:?}", petgraph::dot::Dot::new(.graph))]
    UnexpectedDummyGraphNode { graph: Graph },

    #[error("unexpected dummy ast node")]
    #[diagnostic(
        code(cxx2flow::unexpected_dummy_ast),
        help("dummy node found in the ast\nthis might be a bug, please report it to the author")
    )]
    UnexpectedDummyAstNode {
        #[source_code]
        src: NamedSource,
        #[label("dummy ast node here")]
        range: SourceSpan,
    },

    #[error("unexpected outgoing edge: {node_index:?}, neighbors: {neighbors:?}, graph: {:?}", petgraph::dot::Dot::new(.graph))]
    #[diagnostic(code(cxx2flow::unexpected_outgoing_nodes), help("usually, every dummy node only has one outgoing edge, but this node has zero or more than one outgoing edges\nthis might be a bug, please report it to the author"))]
    UnexpectedOutgoingEdges {
        node_index: NodeIndex,
        neighbors: Vec<NodeIndex>,
        graph: Graph,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
