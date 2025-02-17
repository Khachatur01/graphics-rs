use crate::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Polyline {
    vertices: Vec<Point>
}

impl Polyline {
    pub fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }
}
