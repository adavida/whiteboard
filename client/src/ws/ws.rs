use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

use crate::log;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct Ws {
    ws: WebSocket,
}

impl Ws {
    pub fn get_connect(url: &str) -> Result<Self, JsValue> {
        let ws = WebSocket::new(url)?;

        Ok(Ws { ws: ws })
    }

    pub fn send_msg_string(&self, msg: &str) {
        match self.ws.send_with_str(msg) {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }

    pub fn wait_on_log_new_message(&self) {
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("message event, received Text: {:?}", txt);
                crate::add_text(txt.as_string().expect("not string").as_str());
            } else {
                console_log!("message : {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        let onclose_callback = Closure::wrap(Box::new(|| {
            let closure = Closure::wrap(Box::new(|| {
                console_log!("Timeout");
                let window = web_sys::window().unwrap();
                let _ = window.location().reload();
            }) as Box<dyn FnMut()>);
            let window = web_sys::window().unwrap();
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    3000,
                )
                .unwrap();
            closure.forget();
        }) as Box<dyn FnMut()>);
        self.ws
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        self.ws
            .set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        onclose_callback.forget();
    }
}
