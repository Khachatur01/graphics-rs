use crate::tool::click_draw_tool::ClickDrawTool;
use crate::tool::mouse_events::MouseEvents;
use crate::tool::move_draw_tool::MoveDrawTool;
use crate::tool::select_tool::SelectTool;
use geometry::shape::point::Point;

pub mod move_draw_tool;
pub mod click_draw_tool;
pub mod select_tool;
pub mod mouse_events;

pub enum Tool {
    Select(SelectTool),
    MoveDraw(MoveDrawTool),
    ClickDraw(ClickDrawTool),
}

impl MouseEvents for Tool {
    fn mouse_down(&mut self, point: &Point) {
        match self {
            Tool::MoveDraw(tool) => tool.mouse_down(point),
            Tool::Select(tool) => tool.mouse_down(point),
            Tool::ClickDraw(tool) => tool.mouse_down(point),
        }
    }

    fn mouse_move(&mut self, point: &Point) {
        match self {
            Tool::MoveDraw(tool) => tool.mouse_move(point),
            Tool::Select(tool) => tool.mouse_move(point),
            Tool::ClickDraw(tool) => tool.mouse_move(point),
        }
    }

    fn mouse_up(&mut self, point: &Point) {
        match self {
            Tool::MoveDraw(tool) => tool.mouse_up(point),
            Tool::Select(tool) => tool.mouse_up(point),
            Tool::ClickDraw(tool) => tool.mouse_up(point),
        }
    }
}
