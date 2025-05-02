use crate::tool::draw_tool::click_draw_tool::ClickDrawTool;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

impl<Id> Interactive for ClickDrawTool<Id> {
    fn mouse_down(&mut self, point: &Point) {
        todo!()
    }

    fn mouse_move(&mut self, point: &Point) {
        todo!()
    }

    fn mouse_up(&mut self, point: &Point) {
        todo!()
    }
}

impl<Id> Tool for ClickDrawTool<Id> {}
