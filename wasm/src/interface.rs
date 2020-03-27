use crate::utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_gc_to() -> bool {
    let window = web_sys::window().expect("no global `window` exists");
    let doc = window.document().expect("should have a document on window");    

    let gc = utils::get_value::<f64>("gc_percent_input", &doc).unwrap();

    utils::get_input("a_weight", &doc).unwrap().set_value(&format!("{:.2}", (1.0 - gc) / 2.0));
    utils::get_input("c_weight", &doc).unwrap().set_value(&format!("{:.2}", gc / 2.0));
    utils::get_input("g_weight", &doc).unwrap().set_value(&format!("{:.2}", gc / 2.0));
    utils::get_input("t_weight", &doc).unwrap().set_value(&format!("{:.2}", (1.0 - gc) / 2.0));    
    false
}
