use crate::style::color::Color;

#[derive(Clone)]
pub struct ShapeStyle {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub stroke_width: f64,
    pub stroke_dash_array: Vec<u8>,
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            fill_color: Color(0, 0, 0, 0),
            stroke_color: Color(0, 0, 0, 255),
            stroke_width: 1.0,
            stroke_dash_array: vec![],
        }
    }
}
