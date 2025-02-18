use crate::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Polygon {
    vertices: Vec<Point>
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Polygon {
        Polygon { vertices }
    }
}
