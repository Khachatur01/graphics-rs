use crate::style::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StrokeStyle {
    pub color: Color,
    pub width: f64,
    pub dash_array: Vec<u8>,
    pub antialiasing: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShapeStyle {
    pub fill_color: Color,
    pub stroke: StrokeStyle,
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            fill_color: Color(0, 0, 0, 0),
            stroke: StrokeStyle {
                color: Color(0, 0, 0, 255),
                width: 1.0,
                dash_array: vec![],
                antialiasing: true
            }
        }
    }
}
