#![cfg(target_arch = "wasm32")]

mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    yew::Renderer::<app::App>::new().render();
    log("Hello from the console!");
}
