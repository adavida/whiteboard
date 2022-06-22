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
        let result = Input {
            input: input_element,
        };

        result
    }

    pub fn set_change(&self, ws: &crate::ws::Ws) {
        let onchange_fn = Self::create_on_change_callback(self.input.clone(), ws);
        self.input
            .set_onchange(Some(onchange_fn.as_ref().unchecked_ref()));
        onchange_fn.forget();
    }

    fn create_on_change_callback(
        input: HtmlInputElement,
        ws: &crate::ws::Ws,
    ) -> Closure<dyn FnMut()> {
        let ws_clone = ws.clone();
        Closure::wrap(Box::new(move || {
            let val = input.value().to_string();
            ws_clone.send_client_message(&message::FromClientMessage::ChatMsg(val));
            input.set_value("");
        }) as Box<dyn FnMut()>)
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        log("droper ");
    }
}
