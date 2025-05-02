use crate::geometric::rectangle_view::RectangleElement;
use geometry::figure::point::Point;
use geometry::math::{Drag, Resize};
use std::any::Any;

pub fn mouse_down<Id: 'static>(element: &mut dyn Any, current_point: &Point) {
    let rectangle: &mut RectangleElement<Id> = element.downcast_mut().unwrap();
    rectangle.rectangle.drag(current_point)
}

pub fn mouse_move<Id: 'static>(element: &mut dyn Any, start: &Point, current_point: &Point) {
    let rectangle: &mut RectangleElement<Id> = element.downcast_mut().unwrap();

    let delta: Point = current_point - start;

    rectangle.rectangle.resize(delta.x(), delta.y());
}

pub fn mouse_up<Id: 'static>(element: &mut dyn Any, start: &Point, current_point: &Point) {
    let rectangle: &mut RectangleElement<Id> = element.downcast_mut().unwrap();

    let delta: Point = current_point - start;

    rectangle.rectangle.resize(delta.x(), delta.y());
}
