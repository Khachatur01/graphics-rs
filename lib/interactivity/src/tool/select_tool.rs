mod render;
mod tool;

use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;

pub enum SelectMode {
    Full,
    Intersecting,
}

pub trait Selectable {
    fn select(&self, selection: Rectangle, mode: SelectMode) -> bool;
    fn pick(&self, position: &Point) -> bool;
}

pub struct SelectTool<Id> {
    selected_elements: Vec<Id>,
    selection: Option<Rectangle>,

    get_selectables: Box<dyn Fn() -> Vec<Box<dyn Selectable>>>,
}

impl<Id> SelectTool<Id> {
    pub fn new<GetSelectables>(get_selectables: GetSelectables) -> Self
    where GetSelectables:  Fn() -> Vec<Box<dyn Selectable>> + 'static {
        Self {
            selected_elements: vec![],
            selection: None,
            get_selectables: Box::new(get_selectables),
        }
    }
}
