use getter_methods::GetterMethods;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct Polygon {
    vertices: Vec<Point>
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Polygon {
        Polygon { vertices }
    }
}
