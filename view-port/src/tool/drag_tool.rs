use crate::tool::mouse_events::MouseEvents;
use geometry::shape::point::Point;
use geometry::traits::Drag;

pub struct DragTool {
    element: Box<dyn Drag>,
}

impl DragTool {
    pub fn new(element: Box<dyn Drag>) -> Self {
        Self { element }
    }
}

impl MouseEvents for DragTool {
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
