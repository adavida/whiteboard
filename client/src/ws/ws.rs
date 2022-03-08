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
        let cloned_ws = self.ws.clone();
        let c_m = msg.to_string();

        let onopen_callback =
            Closure::wrap(
                Box::new(move |_| match cloned_ws.send_with_str(c_m.as_str()) {
                    Ok(_) => console_log!("message successfully sent"),
                    Err(err) => console_log!("error sending message: {:?}", err),
                }) as Box<dyn FnMut(JsValue)>,
            );

        self.ws
            .set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    pub fn wait_on_log_new_message(&self) {
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("message event, received Text: {:?}", txt);
            } else {
                console_log!("message : {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        self.ws
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }
}
