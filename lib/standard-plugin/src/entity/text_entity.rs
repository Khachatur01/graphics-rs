mod draw;
mod render;

use getter_methods::GetterMethods;
use rendering::style::text_style::TextStyle;

#[derive(GetterMethods)]
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
