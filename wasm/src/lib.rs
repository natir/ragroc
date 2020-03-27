
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

mod utils;
mod revcomp;
mod generate;
mod interface;

#[wasm_bindgen]
pub fn finish_load() {

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    revcomp::Revcomp::setup();
    generate::Generate::setup();

    /* interface setup */
    let gc_button = document.get_element_by_id("gc_percent").expect("should have gc percent bouton on document");
    gc_button.set_attribute("onclick", "return Ragroc.set_gc_to()").expect("GC% button onclick can't be set");

    /* main functionality setup */
    let generate_form = document.get_element_by_id("generate_param").expect("should have generate form on document");
    generate_form.set_attribute("onsubmit", "return Ragroc.run_generate()").expect("generate form onsubmit can't be set");

    let revcomp_form = document.get_element_by_id("rc_submit").expect("should have generate form on document");
    revcomp_form.set_attribute("onclick", "return Ragroc.run_revcomp()").expect("revcomp button onclick can't be set");
    
    console_error_panic_hook::set_once();
}

    
#[wasm_bindgen]
pub fn run_generate() -> bool {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let generate = generate::Generate::from_document(&document).unwrap();
    
    let output = utils::get_text_area("generate_out_text", &document).unwrap();
    output.set_value(&generate.produce_output());
   
    false
}

#[wasm_bindgen]
pub fn run_revcomp() -> bool {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let revcomp = revcomp::Revcomp::from_document(&document).unwrap();

    let output = utils::get_text_area("rc_out_text", &document).unwrap();
    output.set_value(&revcomp.produce_output());

    false
}
