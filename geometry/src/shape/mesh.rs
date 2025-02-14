use getter_methods::GetterMethods;
use crate::shape::mesh::edge::Edge;
use crate::shape::point::Point;

pub mod edge;

#[derive(GetterMethods)]
pub struct Mesh {
    knots: Vec<Point>,
    edges: Vec<Edge>
}

impl Mesh {
    pub fn new(knots: Vec<Point>, edges: Vec<Edge>) -> Mesh {
        Mesh { knots, edges }
    }
}
