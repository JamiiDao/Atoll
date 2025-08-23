use wasm_bindgen::prelude::*;

/// Makes JS `console.log` available in Rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

/// A demo function to test if WASM is callable from background.js
#[wasm_bindgen]
pub fn hello_wasm() {
    log("Hello from WASM!");
}
