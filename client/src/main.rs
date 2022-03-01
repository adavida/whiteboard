#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::window;

mod compoment;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
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

pub fn get_document() -> web_sys::Document {
    window()
        .and_then(|win| win.document())
        .expect("Could not access document")
}

fn start_app() {
    let document = get_document();
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Bonjour les amis");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
    body.append_child(compoment::first::get_first("david").as_ref())
        .expect("toto");
}

fn main() {
    set_panic_hook();
    start_app();
}
