mod render;
mod tool;

use element_view::ElementView;
use geometry::figure::point::Point;

pub struct ClickDrawTool<Id> {
    points: Vec<Point>,
    drawable: Option<Box<dyn ElementView<Id>>>
}

impl<Id> ClickDrawTool<Id> {
    pub fn new() -> ClickDrawTool<Id> {
        Self { points: Vec::new(), drawable: None }
    }

    pub fn end_drawing(&mut self) {
        self.points.clear();

        /* todo: add copy of drawn element_view to viewport */
    }
}
