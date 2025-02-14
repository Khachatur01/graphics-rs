use getter_methods::GetterMethods;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Circle { center, radius }
    }
}
