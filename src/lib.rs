use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::Element;


#[wasm_bindgen()]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    pub fn querySelector(id: &str) -> Option<Element>;
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
pub fn say_hi() {
    alert("Hi from Rust");
}


#[wasm_bindgen]
pub fn console_log(x: &str) {
    log(x);
}


#[wasm_bindgen]
pub fn update_text(id: &str) {
    if let Some(target) = querySelector(id) {
        target.set_attribute("style", "color: white").unwrap();
        target.set_text_content(Some("I am Rust!"));
    }
}
