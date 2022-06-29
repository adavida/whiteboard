use crate::log;
use message::FromClientMessage;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebSocket;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

type MutexWebsocket = Arc<Mutex<WebSocket>>;
type ActionFn = Rc<dyn Fn(&message::FromServerMessage)>;
#[derive(Clone)]
pub struct Ws {
    ws: MutexWebsocket,
    pub actions: Vec<ActionFn>,
}

fn create_reconnect_callback(ws_arc: MutexWebsocket) -> Closure<dyn std::ops::FnMut()> {
    Closure::wrap(
        Box::new(move || match Ws::create_ws("ws://localhost:8080/ws") {
            Ok(ws_) => {
                let mut ws = ws_arc.lock().unwrap();
                let onmessage = ws.onmessage();
                ws_.set_onmessage(onmessage.as_ref());
                let onclose = ws.onclose();
                ws_.set_onclose(onclose.as_ref());
                console_log!("reconected");
                *ws = ws_;
            }
            _ => try_reconnect(ws_arc.clone()),
        }) as Box<dyn FnMut()>,
    )
}

fn create_onclose_callback(ws_arc: MutexWebsocket) -> Closure<dyn std::ops::FnMut()> {
    Closure::wrap(Box::new(move || try_reconnect(ws_arc.clone())) as Box<dyn FnMut()>)
}

fn set_callback_in_ws(ws_arc: MutexWebsocket) {
    let ws = ws_arc.lock().unwrap();
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

    pub fn add_action(&mut self, callback: ActionFn) {
        self.actions.push(callback);
        self.set_onmessage_def_callback();
    }

    pub fn get_connect(url: &str) -> Self {
        let new_ws = Arc::new(Mutex::new(
            Self::create_ws(url).expect("cannot connect to websocket"),
        ));
        set_callback_in_ws(new_ws.clone());
        let result = Ws {
            ws: new_ws,
            actions: vec![],
        };
        result.set_onmessage_def_callback();

        result
    }

    pub fn send_client_message(&self, msg: &FromClientMessage) {
        match self
            .get_ws()
            .lock()
            .unwrap()
            .send_with_str(msg.to_serialized().as_str())
        {
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

    pub fn set_onmessage_def_callback(&self) {
        let actions = self.actions.clone();
        let callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let entry_message_o = message::FromServerMessage::from_serialized(
                    txt.as_string().expect("not string"),
                );
                if let Ok(entry_message) = entry_message_o {
                    for action in actions.iter() {
                        action(&entry_message);
                    }
                } else {
                    console_log!("error in message : {txt}")
                }
            }
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);

        self.set_onmessage_callback(callback);
    }

    fn get_ws(&self) -> MutexWebsocket {
        self.ws.clone()
    }
}
