use crate::style::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SurfaceStyle {
    pub fill_color: Color,
}

impl Default for SurfaceStyle {
    fn default() -> Self {
        Self {
            fill_color: Color(0, 0, 0, 0),
        }
    }
}
