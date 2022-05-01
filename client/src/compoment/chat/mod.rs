use web_sys::{Document, Element, HtmlElement};

static H2_TEXT: &str = "Chat Box";

#[derive(Clone)]
pub struct ChatBox {
    document: Document,
    element: Element,
}

impl ChatBox {
    pub fn create(document_: Document, parent_element: &HtmlElement) -> Self {
        let new_element = ChatBox::create_element(&document_);

        parent_element
            .append_child(new_element.as_ref())
            .expect("cannot add chat box");

        ChatBox {
            document: document_,
            element: new_element,
        }
    }

    pub fn new_message(&self, message: &str) {
        let div = self
            .document
            .create_element("div")
            .expect("chatbox: cannot create br element");
        let text_node = self.document.create_text_node(message.as_ref());
        div.append_child(text_node.as_ref())
            .expect("chatbox: cannot add text");

        self.element
            .append_child(div.as_ref())
            .expect("chatbox: cannot add text");
        div.scroll_into_view();
    }

    fn create_element(document: &Document) -> Element {
        let main = document
            .create_element("div")
            .expect("cannot create chat main element");
        main.set_id("main_chat");
        main.append_child(ChatBox::create_head(document).as_ref())
            .expect("cannot append head");
        main
    }

    fn create_head(document: &Document) -> Element {
        let h2 = document.create_element("h2").expect("cannot create h2");
        h2.set_inner_html(H2_TEXT);
        h2
    }
}
