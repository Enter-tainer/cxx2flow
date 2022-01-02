use tree_sitter_facade;
#[cfg(not(target_arch = "wasm32"))]
pub async fn get_lang() -> tree_sitter_facade::Language {
    tree_sitter_facade::Language::from(tree_sitter_cpp::language())
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
#[cfg(target_arch = "wasm32")]
pub async fn get_lang() -> tree_sitter_facade::Language {
    init_panic_hook();
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_tree_sitter_sys::Language;
    let bytes: &[u8] = include_bytes!("../assets/tree-sitter-cpp.wasm");
    let lang = JsFuture::from(Language::load_bytes(&bytes.into())).await;
    if let Err(lang) = lang {
      panic!("{:?}", lang.as_string())
    }
    let lang = lang.unwrap();
    tree_sitter_facade::Language::from(lang.unchecked_into::<Language>())
}
