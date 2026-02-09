use wasm_bindgen::prelude::*;

use crate::{
    display::{dot::Dot, GraphDisplayBackend},
    generate,
};

#[wasm_bindgen]
pub fn generate_dot(content: &str, function_name: Option<String>, curly: bool) -> Result<String, JsValue> {
    generate(
        content.as_bytes(),
        "input.cpp",
        function_name,
        GraphDisplayBackend::Dot(Dot::new(curly)),
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))
}

