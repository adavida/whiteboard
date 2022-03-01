use super::super::get_document;

pub fn get_first(name: &str) -> web_sys::Element {
    let document = get_document();
    let text = format!("Bonjour {name}");
    let element = document.create_element("div").expect("e");
    let main = document.create_text_node(&text);
    element.append_child(main.as_ref()).expect("ici");
    element
}
