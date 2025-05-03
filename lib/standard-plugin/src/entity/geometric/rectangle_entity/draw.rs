use crate::entity::geometric::rectangle_entity::RectangleEntity;
use core::entity::Entity;
use geometry::figure::point::Point;
use geometry::math::{Drag, Resize};

pub fn mouse_down<Id>(element: &mut Entity<Id>, current_point: &Point) {
    let rectangle: &mut RectangleEntity = element.model_ref_mut();
    rectangle.rectangle.drag(current_point)
}

pub fn mouse_move<Id>(element: &mut Entity<Id>, start: &Point, current_point: &Point) {
    let rectangle: &mut RectangleEntity = element.model_ref_mut();

    let delta: Point = current_point - start;

    rectangle.rectangle.resize(delta.x(), delta.y());
}

pub fn mouse_up<Id>(element: &mut Entity<Id>, start: &Point, current_point: &Point) {
    let rectangle: &mut RectangleEntity = element.model_ref_mut();

    let delta: Point = current_point - start;

    rectangle.rectangle.resize(delta.x(), delta.y());
}
