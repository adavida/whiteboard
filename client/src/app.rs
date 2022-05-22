use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::ws::Ws;

pub struct App;
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}


fn create_refesh_button() -> Element {
    let document = crate::get_document();
    let button = document
        .create_element("button")
        .expect("cannot create button");
    button.set_inner_html("click  me for refresh");
    let call_action = Closure::wrap(Box::new(|| crate::reload()) as Box<dyn Fn()>);
    button
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("lol")
        .set_onclick(Some(call_action.as_ref().unchecked_ref()));
    call_action.forget();
    button
}

fn create_ws(chat_box: crate::compoment::chat::ChatBox) -> Ws{
    let ws = Ws::get_connect("ws://localhost:8080/ws");

    let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let entry_message_o =
                message::Message::from_serialized(txt.as_string().expect("not string"));
            if let Ok(entry_message) = entry_message_o {
                match entry_message {
                    message::Message::Chat(chat_message) => {
                        chat_box.new_message(chat_message.as_str())
                    }
                    message::Message::Reload => crate::reload(),
                }
            } else {
                console_log!("error in message : {txt}")
            }
        }
    }) as Box<dyn FnMut(web_sys::MessageEvent)>);

    ws.set_onmessage_callback(onmessage_callback);
    ws
}

impl App {
    pub fn new() -> Self {
        let document = crate::get_document();
        let body = document.body().expect("Could not access document.body");

        let refresh_button = create_refesh_button();
        body.append_child(refresh_button.as_ref()).expect("Can not create reload button");
        let chat_box = crate::compoment::chat::ChatBox::create(document, &body);
        let input = crate::compoment::input::Input::new();

        let ws = create_ws(chat_box);
        input.set_change(&ws);
        body.append_child(input.input.as_ref()).expect("titi");
        App
    }
}