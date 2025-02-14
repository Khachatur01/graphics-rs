use crate::tool::click_draw_tool::ClickDrawTool;
use crate::tool::drag_tool::DragTool;
use crate::tool::mouse_events::MouseEvents;
use crate::tool::move_draw_tool::MoveDrawTool;
use geometry::shape::point::Point;

pub mod move_draw_tool;
pub mod click_draw_tool;
pub mod drag_tool;
pub mod mouse_events;

pub enum Tool {
    MoveDraw(MoveDrawTool),
    ClickDraw(ClickDrawTool),
    Drag(DragTool)
}

impl MouseEvents for Tool {
    fn make_mouse_down(&mut self, point: &Point) {
        match self {
            Tool::MoveDraw(tool) => tool.make_mouse_down(point),
            Tool::Drag(tool) => tool.make_mouse_down(point),
            Tool::ClickDraw(tool) => tool.make_mouse_down(point),
        }
    }

    fn make_mouse_move(&mut self, point: &Point) {
        match self {
            Tool::MoveDraw(tool) => tool.make_mouse_move(point),
            Tool::Drag(tool) => tool.make_mouse_move(point),
            Tool::ClickDraw(tool) => tool.make_mouse_move(point),
        }
    }

    fn make_mouse_up(&mut self, point: &Point) {
        match self {
            Tool::MoveDraw(tool) => tool.make_mouse_up(point),
            Tool::Drag(tool) => tool.make_mouse_up(point),
            Tool::ClickDraw(tool) => tool.make_mouse_up(point),
        }
    }
}
