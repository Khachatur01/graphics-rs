use algebra::linear::vector::Vector;

#[derive(Clone)]
pub struct Polyline<const D: usize> {
    vertices: Vec<Vector<D>>,
}

impl<const D: usize> Polyline<D> {
    pub fn new(vertices: &[Vector<D>]) -> Self {
        Self { vertices: vertices.to_vec() }
    }
}
