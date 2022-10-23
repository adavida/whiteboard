use std::rc::Rc;

use crate::get_document;
use crate::ws::Ws;
use message::FromClientMessage;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::HtmlLabelElement;

pub struct App {
    document: Document,
    body: HtmlElement,
    login: Option<u32>,
    ws: Ws,
}

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

fn create_ws() -> Ws {
    let ws = Ws::get_connect("ws://localhost:8080/ws");
    ws
}

impl App {
    pub fn new() -> Self {
        let document = crate::get_document();

        let body = document.body().expect("Could not access document.body");
        let mut app = Self {
            document: document,
            body: body,
            login: None,
            ws: create_ws(),
        };
        app.display();

        app
    }

    fn display(&mut self) {
        self.clear_page();

        match self.login {
            Some(_) => self.display_main_page(),
            None => self.display_login_page(),
        }
        self.display_main_page();
    }

    fn clear_page(&self) {
        self.body.set_inner_html("");
    }

    fn display_login_page(&self) {
        let document = get_document();
        let label = document
            .create_element("label")
            .expect(r#"canot create login label"#)
            .dyn_into::<HtmlLabelElement>()
            .unwrap();
        self.body
            .append_child(label.as_ref())
            .expect("canot add label");
        label.set_inner_text("name : ");
        let input = crate::compoment::input::Input::new();
        label
            .append_child(input.input.as_ref())
            .expect("canot add input on login page");
        let button = document
            .create_element("input")
            .expect("canot create entry button")
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        button.set_type("submit");
        button.set_value("entry");
        let ws = self.ws.clone();

        let button_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let msg = FromClientMessage::Login(input.input.value());
            ws.send_client_message(&msg);
            console_log!("name : {}", input.input.value());
        }) as Box<dyn FnMut(_)>);

        button.set_onclick(Some(button_callback.as_ref().unchecked_ref()));
        button_callback.forget();
        self.body.append_child(button.as_ref()).unwrap();
    }

    fn display_main_page(&mut self) {
        let refresh_button = create_refesh_button();
        self.body
            .append_child(refresh_button.as_ref())
            .expect("Can not create reload button");
        let chat_box = crate::compoment::chat::ChatBox::create(self.document.clone(), &self.body);

        let callback = move |message: &message::FromServerMessage| {
            if let message::FromServerMessage::Chat(msg) = message {
                chat_box.new_message(msg.as_str());
            }
        };
        self.ws.add_action(Rc::from(callback));

        let input = crate::compoment::input::Input::new();
        input.set_change(&self.ws);
        self.body.append_child(input.input.as_ref()).expect("titi");
    }
}
