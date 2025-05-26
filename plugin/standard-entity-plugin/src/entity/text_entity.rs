use getter_methods::GetterMethods;
use core_derive::Model;
use standard_rendering_plugin::style::text_style::TextStyle;

#[derive(Model, Clone, GetterMethods)]
pub struct TextEntity {
    text: String,
    style: TextStyle,
}

impl TextEntity {
    pub fn new(text: String) -> Self {
        Self {
            text,
            style: TextStyle::default(),
        }
    }
}
