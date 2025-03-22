mod render;
mod tool;

use geometry::figure::rectangle::Rectangle;
use geometry::math::Selectable;

pub struct SelectTool {
    selected_elements: Vec<Box<dyn Selectable>>,
    selection: Option<Rectangle>,
}

impl SelectTool {
    pub fn new() -> Self {
        Self {
            selected_elements: vec![],
            selection: None,
        }
    }
}
