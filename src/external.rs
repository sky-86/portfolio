use wasm_bindgen::prelude::*;

// binds javascript function to a rust function
// this function is what provides syntax highlighting to code snippets
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    pub fn highlightAll();
}
