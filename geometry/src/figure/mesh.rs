use crate::figure::mesh::edge::Edge;
use algebra::linear::vector::Vector;

pub mod edge;

#[derive(Clone)]
pub struct Mesh<const D: usize> {
    knots: Vector<D>,
    edges: Vec<Edge>,
}

impl<const D: usize> Mesh<D> {
    pub fn new(knots: Vector<D>, edges: Vec<Edge>) -> Mesh<D> {
        Mesh { knots, edges }
    }
}
