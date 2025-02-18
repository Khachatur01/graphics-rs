use crate::figure::mesh::edge::Edge;
use crate::figure::point::Point;
use getter_methods::GetterMethods;

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
