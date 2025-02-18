use crate::element::text_element::TextElement;
use geometry::figure::point::Point;
use interactivity::tool::draw_tool::draw::Draw;

impl<Id> Draw for TextElement<Id> {
    fn mouse_down(&mut self, points: &Vec<Point>, mouse_position: &Point) {
        todo!()
    }

    fn mouse_move(&mut self, points: &Vec<Point>, mouse_position: &Point) {
        todo!()
    }

    fn mouse_up(&mut self, points: &Vec<Point>, mouse_position: &Point) {
        todo!()
    }
}
