use crate::element::geometric_element::{GeometricElement, GeometricFigure};
use geometry::figure::point::Point;
use interactivity::tool::draw_tool::draw::Draw;

impl<Id> Draw for GeometricElement<Id> {
    fn mouse_down(&mut self, points: &Vec<Point>, mouse_position: &Point) {
        match &mut self.geometric_figure {
            GeometricFigure::Point(point) => todo!(),
            GeometricFigure::Segment(segment) => todo!(),
        }
    }

    fn mouse_move(&mut self, points: &Vec<Point>, mouse_position: &Point) {
        match &mut self.geometric_figure {
            GeometricFigure::Point(point) => todo!(),
            GeometricFigure::Segment(segment) => todo!(),
        }
    }

    fn mouse_up(&mut self, points: &Vec<Point>, mouse_position: &Point) {
        match &mut self.geometric_figure {
            GeometricFigure::Point(point) => todo!(),
            GeometricFigure::Segment(segment) => todo!(),
        }
    }
}
