mod ast;
pub mod cli;
pub mod display;
pub mod dump;
pub mod error;
mod graph;
mod parser;
use display::{GraphDisplay, GraphDisplayBackend};
use error::Result;

pub fn generate(
    content: &[u8],
    file_name: &str,
    function_name: Option<String>,
    backend: GraphDisplayBackend,
) -> Result<String> {
    let ast = parser::parse(content, file_name, function_name)?;
    // dbg!(&ast);
    let graph = graph::from_ast(ast, &String::from_utf8(content.to_vec())?, file_name)?;
    // dbg!(&graph);
    backend.generate_from_graph(&graph)
}

// WASM-specific bindings
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_init() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn convert_to_flowchart(
    code: &str,
    function_name: Option<String>,
    use_curly: bool,
    use_tikz: bool,
    use_d2: bool,
) -> std::result::Result<String, JsValue> {
    let backend = if use_tikz {
        GraphDisplayBackend::Tikz
    } else if use_d2 {
        GraphDisplayBackend::D2
    } else if use_curly {
        GraphDisplayBackend::DotCurve
    } else {
        GraphDisplayBackend::Dot
    };
    
    generate(code.as_bytes(), "input.cpp", function_name, backend)
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
}
