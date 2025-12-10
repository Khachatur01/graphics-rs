use crate::point::point_2d::Point2D;
use getter_methods::GetterMethods;

#[derive(GetterMethods, Copy, Clone)]
pub struct Circle {
    center: Point2D,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point2D, radius: f64) -> Circle {
        Circle { center, radius }
    }
}
