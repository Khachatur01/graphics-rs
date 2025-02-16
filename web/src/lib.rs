mod utils;

use view_port::view_port::ViewPort;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebViewPort {
    view_port: ViewPort
}

#[wasm_bindgen]
impl WebViewPort {
    #[wasm_bindgen]
    pub fn new() -> Self {
        Self {
            view_port: ViewPort::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_element(&mut self) {
        
    }
}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, web!");
}
