use crate::style::color::Color;

#[derive(Clone)]
pub struct TextStyle {
    pub font_color: Color,
    pub background_color: Color,
    pub font_size: u32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_color: Color(0, 0, 0, 255),
            background_color: Color(0, 0, 0, 0),
            font_size: 11,
        }
    }
}
