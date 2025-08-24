use wasm_bindgen::{JsCast, prelude::*};
use web_sys::js_sys;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello_wasm() {
    log("Atoll Wallet Extension Online...:)");
}

#[wasm_bindgen]
pub fn app_ready() {
    log("Event fired...:)");
}
