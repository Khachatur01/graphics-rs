use crate::tool::mouse_events::MouseEvents;
use crate::tool::Tool;
use crate::view_port::element_id::ElementId;
use crate::view_port::element_view::ElementView;
use geometry::shape::point::Point;

pub mod element_view;
pub mod element_id;

pub struct ViewPort {
    elements: Vec<ElementView<ElementId>>,
    active_tool: Option<Tool>,
}

impl ViewPort {
    pub fn new() -> Self {
        Self {
            elements: vec![],
            active_tool: None,
        }
    }

    pub fn activate_tool(&mut self, tool: Tool) {
        self.active_tool = Some(tool);
    }

    pub fn add_element(&mut self, element: ElementView<ElementId>) {
        self.elements.push(element);
    }
}

impl MouseEvents for ViewPort {
    fn mouse_down(&mut self, point: &Point) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.mouse_down(point);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.mouse_move(point);
    }

    fn mouse_up(&mut self, point: &Point) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.mouse_up(point);
    }
}
