use wasm_bindgen::prelude::*;
use web_sys::{Element, Document, MouseEvent, window, Event, HtmlInputElement};


#[wasm_bindgen()]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    pub fn querySelector(id: &str) -> Option<Element>;

    #[wasm_bindgen(js_namespace = document)]
    pub fn createElement(element: &str) -> Option<Element>;
}




#[wasm_bindgen]
pub fn typed_arr(arr: Box<[i32]>) -> Box<[i32]> {
    arr
}

#[wasm_bindgen]
pub fn rsult_enum(is_ok: bool) -> Result<i32, JsValue> {
    if !is_ok { return Err("Some error".into()) }
    Ok(12)
}

#[wasm_bindgen]
pub struct Cat {
    name: String
}

#[wasm_bindgen]
impl Cat {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Cat {
        Cat { name }
    }

    pub fn say(&self) -> String {
        "Mhwww".to_owned()
    }

    pub fn transfer_ownership(self) -> Cat {
        self
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}


#[wasm_bindgen]
pub fn update_text(id: &str) {
    if let Some(target) = querySelector(id) {
        target.set_attribute("style", "color: white").unwrap();
        target.set_text_content(Some("I am Rust!"));
    }
}


fn create_button(
    document: &Document,
    text: Option<&str>,
    class: &str
) -> Element {
    let ele = document.create_element("div").expect("Failed create div element");
    let ctx = document.create_element("p").expect("Failed create p element");

    ctx.set_text_content(text);

    ele.set_attribute("class", class).expect("Failed to set class attribute element");
    ele.set_attribute("onclick", &format!("show('{}')", text.expect("Failed: text is null"))).expect("Failed to set onclick attribute.");

    ele
}


#[wasm_bindgen()]
pub fn app() {
    let window = window().expect("Failed to load window");
    let document = window.document().expect("Failed to load document");

    // Elements
    let root = document.query_selector("#root")
        .expect("Failed to select root element")
        .expect("Not found root element");

    let container = document
        .create_element("div")
        .expect("Failed to create div element");
    let cal = document
        .create_element("div")
        .expect("Failed to create div element");

    root.append_child(&container)
        .expect("Failed to append container");
    container.append_child(&cal)
        .expect("Failed to append cal");

    let input = document
        .create_element("input")
        .expect("Failed to creeate input element");
    input.set_attribute("placeholder", "some text").expect("Failed to set placeholder input");

    let button_container = document
        .create_element("div")
        .expect("Failed to create button container");

    cal.append_child(&input)
        .expect("Failed to append input");
    cal.append_child(&button_container)
        .expect("Failed to append button_container");

    let ac = create_button(&document, Some("AC"), "btn primary-operator ac");

    let handle_change_input = Closure::wrap(Box::new(move |evt: Event| {
        let value = evt.target().expect("Failed to load target").dyn_into::<HtmlInputElement>().expect("Failed to load input element").value();
        log(&value);
    }) as Box<dyn FnMut(Event)>);
    input.add_event_listener_with_callback("input", &handle_change_input.as_ref().unchecked_ref()).expect("Failed to set event listener");
    handle_change_input.forget();
}
