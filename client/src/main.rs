#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlInputElement;

mod compoment;
mod ws;

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

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn get_document() -> web_sys::Document {
    window()
        .and_then(|win| win.document())
        .expect("Could not access document")
}

pub fn add_text(text: &str) {
    let document = get_document();
    let body = document.body().expect("eee");
    let br = document
        .create_element("br")
        .expect("cannot create br element");
    let text_node = document.create_text_node(text.as_ref());
    body.append_child(br.as_ref())
        .expect("cannot add br element in body");
    body.append_child(text_node.as_ref()).expect("e1");
}

pub fn reload() {
    window()
        .expect("no windows")
        .location()
        .reload()
        .expect("cannot reload page")
}

fn start_app() {
    let document = get_document();
    let body = document.body().expect("Could not access document.body");

    let button = document
        .create_element("button")
        .expect("cannot create button");
    button.set_inner_html("click  me for refresh");
    let call_action = Closure::wrap(Box::new(|| reload()) as Box<dyn Fn()>);
    button
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("lol")
        .set_onclick(Some(call_action.as_ref().unchecked_ref()));
    call_action.forget();
    body.append_child(button.as_ref()).expect("toto");

    let chat_box = compoment::chat::ChatBox::create(document, &body);

    let input = compoment::input::Input::new();
    let ws = ws::Ws::get_connect("ws://localhost:8080/ws");

    let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let entry_message_o =
                message::Message::from_serialized(txt.as_string().expect("not string"));
            if let Ok(entry_message) = entry_message_o {
                match entry_message {
                    message::Message::Chat(chat_message) => {
                        chat_box.new_message(chat_message.as_str())
                    }
                    message::Message::Reload => reload(),
                }
            } else {
                console_log!("error in message : {txt}")
            }
        }
    }) as Box<dyn FnMut(web_sys::MessageEvent)>);

    ws.set_onmessage_callback(onmessage_callback);
    let input_clone: HtmlInputElement = input.input.clone();
    let on_change = Closure::wrap(Box::new(move || {
        let val = input_clone.value();
        ws.send_msg_string(val.as_str());
        input_clone.set_value("");
    }) as Box<dyn FnMut()>);
    input.set_change(on_change);
    body.append_child(input.input.as_ref()).expect("titi");
}

fn main() {
    set_panic_hook();
    start_app();
}
