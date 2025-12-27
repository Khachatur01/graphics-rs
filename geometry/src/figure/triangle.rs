use algebra::linear::vector::Vector;

#[derive(Clone)]
pub struct Triangle<const D: usize> {
    p0: Vector<D>,
    p1: Vector<D>,
    p2: Vector<D>,
}
