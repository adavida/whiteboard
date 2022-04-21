use super::super::get_document;
use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub struct Input {
    pub input: HtmlInputElement,
}

impl Input {
    pub fn new() -> Self {
        let document = get_document();
        let input_element = document
            .create_element("input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        Input {
            input: input_element,
        }
    }

    pub fn set_change(&self, onchange_fn: Closure<dyn std::ops::FnMut()>) {
        self.input
            .set_onchange(Some(onchange_fn.as_ref().unchecked_ref()));
        onchange_fn.forget();
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        log("droper ");
    }
}
