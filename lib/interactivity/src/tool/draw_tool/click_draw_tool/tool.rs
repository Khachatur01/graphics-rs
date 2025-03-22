use crate::tool::draw_tool::click_draw_tool::ClickDrawTool;
use crate::tool::draw_tool::draw_mode::ClickDraw;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

impl<Drawable: ClickDraw> Interactive for ClickDrawTool<Drawable> {
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

impl<Drawable: ClickDraw> Tool for ClickDrawTool<Drawable> {}
