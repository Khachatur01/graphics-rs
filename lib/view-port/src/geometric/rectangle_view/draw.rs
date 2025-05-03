use crate::geometric::rectangle_view::RectangleElement;
use element_view::Element;
use geometry::figure::point::Point;
use geometry::math::{Drag, Resize};

pub fn mouse_down<Id: 'static>(element: &mut Element<Id, RectangleElement>, current_point: &Point) {
    let rectangle = element.downcast_ref_mut().unwrap();
    rectangle.rectangle.drag(current_point)
}

pub fn mouse_move<Id: 'static>(element: &mut Element<Id, RectangleElement>, start: &Point, current_point: &Point) {
    let rectangle = element.downcast_ref_mut().unwrap();

    let delta: Point = current_point - start;

    rectangle.rectangle.resize(delta.x(), delta.y());
}

pub fn mouse_up<Id: 'static>(element: &mut Element<Id, RectangleElement>, start: &Point, current_point: &Point) {
    let rectangle = element.downcast_ref_mut().unwrap();

    let delta: Point = current_point - start;

    rectangle.rectangle.resize(delta.x(), delta.y());
}
