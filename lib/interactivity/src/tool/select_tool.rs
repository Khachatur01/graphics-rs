mod render;
mod tool;

use geometry::figure::rectangle::Rectangle;

pub enum Selection {
    Full,
    Intersecting,
}

pub trait Selectable {
    fn select(&mut self, selection: Rectangle) -> bool;
}

pub struct SelectTool<Id> {
    selected_elements: Vec<Id>,
    selection: Option<Rectangle>,
}

impl<Id> SelectTool<Id> {
    pub fn new() -> Self {
        Self {
            selected_elements: vec![],
            selection: None,
        }
    }
}
