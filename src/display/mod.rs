use enum_dispatch::enum_dispatch;

use crate::{error::Result, graph::Graph};

use self::d2::D2;
use self::dot::Dot;
use self::tikz::Tikz;

pub mod d2;
pub mod dot;
pub mod tikz;
#[enum_dispatch]
pub enum GraphDisplayBackend {
    Dot,
    Tikz,
    D2,
}
#[enum_dispatch(GraphDisplayBackend)]
pub trait GraphDisplay {
    fn generate_from_graph(&self, graph: &Graph) -> Result<String>;
}
