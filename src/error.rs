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
  GarbageToken(&'static str)
}

pub type Result<T> = std::result::Result<T, Error>;
