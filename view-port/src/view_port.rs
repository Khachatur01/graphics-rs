use crate::tool::mouse_events::MouseEvents;
use crate::tool::Tool;
use crate::view_port::element_view::ElementView;
use geometry::shape::point::Point;

pub mod element_view;

pub struct ViewPort {
    elements: Vec<ElementView<String>>,
    active_tool: Tool,
}

impl ViewPort {
    pub fn new(active_tool: Tool) -> Self {
        Self {
            elements: vec![],
            active_tool,
        }
    }

    pub fn activate_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }
}

impl MouseEvents for ViewPort {
    fn make_mouse_down(&mut self, point: &Point) {
        self.active_tool.make_mouse_down(point);
    }

    fn make_mouse_move(&mut self, point: &Point) {
        self.active_tool.make_mouse_move(point);
    }

    fn make_mouse_up(&mut self, point: &Point) {
        self.active_tool.make_mouse_up(point);
    }
}
