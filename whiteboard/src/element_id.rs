use std::fmt::Display;
use view_port::identifier::Identifier;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ElementId {
    owner_id: String,
    index: usize,
}

impl Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}_{}", self.owner_id, self.index))
    }
}

impl Identifier for ElementId {
    fn generate() -> Self {
        ElementId {
            owner_id: "asdasd".to_string(),
            index: js_sys::Date::new_0().get_milliseconds() as usize,
        }
    }
}
