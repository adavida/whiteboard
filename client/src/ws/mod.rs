use crate::log;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebSocket;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

type MutexWebsocket = Arc<Mutex<WebSocket>>;

pub struct Ws {
    ws: MutexWebsocket,
}

// fn create_onmessage_callback() -> Closure<dyn std::ops::FnMut(web_sys::MessageEvent)> {
//     Closure::wrap(Box::new(|e: MessageEvent| {
//         if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
//             crate::add_text(txt.as_string().expect("not string").as_str());
//         }
//     }) as Box<dyn FnMut(MessageEvent)>)
// }

fn create_reconnect_callback(ws_arc: MutexWebsocket) -> Closure<dyn std::ops::FnMut()> {
    Closure::wrap(Box::new(move || {
        match Ws::create_ws("ws://localhost:8080/ws") {
            Ok(ws_) => {
                let mut ws = ws_arc.lock().unwrap();
                *ws = ws_;
            }
            _ => try_reconnect(ws_arc.clone()),
        }
        set_callback_in_ws(ws_arc.clone());
    }) as Box<dyn FnMut()>)
}

fn create_onclose_callback(ws_arc: MutexWebsocket) -> Closure<dyn std::ops::FnMut()> {
    Closure::wrap(Box::new(move || try_reconnect(ws_arc.clone())) as Box<dyn FnMut()>)
}

fn set_callback_in_ws(ws_arc: MutexWebsocket) {
    let ws = ws_arc.lock().unwrap();
    // let onmessage_callback = create_onmessage_callback();
    // ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // onmessage_callback.forget();
    let onclose_callback = create_onclose_callback(ws_arc.clone());
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
}

fn try_reconnect(ws_arc: MutexWebsocket) {
    console_log!("try reconnect");
    let closure = create_reconnect_callback(ws_arc);
    let window = web_sys::window().unwrap();
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )
        .unwrap();
    closure.forget();
}

impl Ws {
    fn create_ws(url: &str) -> Result<WebSocket, JsValue> {
        let ws = WebSocket::new(url)?;
        Ok(ws)
    }

    pub fn get_connect(url: &str) -> Self {
        let new_ws = Arc::new(Mutex::new(
            Self::create_ws(url).expect("cannot connect to websocket"),
        ));
        set_callback_in_ws(new_ws.clone());
        Ws { ws: new_ws }
    }

    pub fn send_msg_string(&self, msg: &str) {
        match self.get_ws().lock().unwrap().send_with_str(msg) {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }

    pub fn set_onmessage_callback(
        &self,
        callback: Closure<dyn std::ops::FnMut(web_sys::MessageEvent)>,
    ) {
        self.get_ws()
            .lock()
            .unwrap()
            .set_onmessage(Some(callback.as_ref().unchecked_ref()));

        callback.forget();
    }

    fn get_ws(&self) -> MutexWebsocket {
        self.ws.clone()
    }
}
