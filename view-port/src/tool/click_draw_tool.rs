use crate::tool::mouse_events::MouseEvents;
use geometry::shape::point::Point;
use geometry::traits::{AddPoint, DragPoint, RemovePoint};

trait ClickDrawable: AddPoint + RemovePoint + DragPoint {}

pub struct ClickDrawTool {
    element: Box<dyn ClickDrawable>,
}

impl ClickDrawTool {
    pub fn new(element: Box<dyn ClickDrawable>) -> Self {
        Self { element }
    }
}

impl MouseEvents for ClickDrawTool {
    fn make_mouse_down(&mut self, point: &Point) {
        todo!()
    }

    fn make_mouse_move(&mut self, point: &Point) {
        todo!()
    }

    fn make_mouse_up(&mut self, point: &Point) {
        todo!()
    }
}
