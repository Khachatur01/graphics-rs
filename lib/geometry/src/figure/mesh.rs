use crate::figure::mesh::edge::Edge;
use getter_methods::GetterMethods;

pub mod edge;

#[derive(GetterMethods)]
pub struct Mesh<P> {
    knots: Vec<P>,
    edges: Vec<Edge>,
}

impl<P> Mesh<P> {
    pub fn new(knots: Vec<P>, edges: Vec<Edge>) -> Mesh<P> {
        Mesh { knots, edges }
    }
}
