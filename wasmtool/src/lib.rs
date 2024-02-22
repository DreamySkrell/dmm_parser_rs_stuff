mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasmtool!");
}

#[wasm_bindgen]
pub fn autopipe(origin_str: String) -> String {
    let result_str = dmm_parser_rs::autopipe::autopipe(origin_str);
    result_str
}
