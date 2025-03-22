use crate::tool::select_tool::SelectTool;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::Resize;

impl Interactive for SelectTool {
    fn mouse_down(&mut self, point: &Point) {
        self.selection = Some(
            Rectangle::zero_sized(point.clone())
        );
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(selection) = &mut self.selection else {
            return;
        };

        let delta: Point = point - selection.top_left();
        selection.resize(delta.x(), delta.y());
    }

    fn mouse_up(&mut self, _: &Point) {
        self.selection = None;
    }
}

impl Tool for SelectTool {}
