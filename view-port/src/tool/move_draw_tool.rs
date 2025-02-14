use crate::tool::mouse_events::MouseEvents;
use geometry::shape::point::Point;
use geometry::traits::Resize;

trait MoveDrawable: Resize {}

pub struct MoveDrawTool {
    element: Box<dyn MoveDrawable>,
}

impl MoveDrawTool {
    pub fn new(element: Box<dyn MoveDrawable>) -> Self {
        Self { element }
    }
}

impl MouseEvents for MoveDrawTool {
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
