use enum_dispatch::enum_dispatch;

use crate::{error::Result, graph::Graph};

use self::dot::Dot;
use self::tikz::Tikz;

pub mod dot;
pub mod tikz;

#[enum_dispatch]
pub enum GraphDisplayBackend {
    Dot,
    Tikz,
}
#[enum_dispatch(GraphDisplayBackend)]
pub trait GraphDisplay {
    fn generate_from_graph(&self, graph: &Graph) -> Result<String>;
}
