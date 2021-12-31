use petgraph::graph::NodeIndex;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
  #[error("Treesitter error")]
  TreeSitter(#[from] tree_sitter::LanguageError),
  #[error("IO error")]
  Io(#[from] std::io::Error),
  #[error("UTF-8 error")]
  UTF8(#[from] std::str::Utf8Error),
  #[error("{0} not found")]
  NotFound(&'static str),
  #[error("garbage token {0}")]
  GarbageToken(&'static str),
  #[error("unexpected continue")]
  UnexpectedContinue, // TODO: 尝试未来把行号塞进去？
  #[error("unexpected break")]
  UnexpectedBreak,
  #[error("unexpected dummy graph node")]
  UnexpectedDummyGraphNode,
  #[error("unexpected dummy ast node")]
  UnexpectedDummyAstNode,
  #[error("unexpected outgoing nodes {node_index:?} : {neighbors:?}")]
  UnexpectedOutgoingNodes{node_index: NodeIndex, neighbors: Vec<NodeIndex>}
}

pub type Result<T> = std::result::Result<T, Error>;
