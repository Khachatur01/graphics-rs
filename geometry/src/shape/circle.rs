use crate::shape::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods, Copy, Clone)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Circle { center, radius }
    }
}
