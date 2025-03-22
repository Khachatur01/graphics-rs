use crate::element_view::geometric::rectangle_view::RectangleElement;
use crate::identifier::Identifier;
use geometry::figure::point::Point;
use geometry::math::{Drag, Resize};
use interactivity::tool::draw_tool::draw_mode::MoveDraw;

impl<Id: Identifier> MoveDraw for RectangleElement<Id> {
    fn mouse_down(&mut self, current_point: &Point) {
        self.rectangle.drag(current_point)
    }

    fn mouse_move(&mut self, start: &Point, current_point: &Point) {
        let delta: Point = current_point - start;

        self.rectangle.resize(delta.x(), delta.y());
    }

    fn mouse_up(&mut self, start: &Point, current_point: &Point) {
        let delta: Point = current_point - start;

        self.rectangle.resize(delta.x(), delta.y());
    }
}
