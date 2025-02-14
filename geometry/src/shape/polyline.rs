use getter_methods::GetterMethods;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct Polyline {
    vertices: Vec<Point>
}

impl Polyline {
    pub fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }
}
