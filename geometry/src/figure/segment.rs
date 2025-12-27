use algebra::linear::vector::Vector;

#[derive(Clone)]
pub struct Segment<const D: usize> {
    start: Vector<D>,
    end: Vector<D>,
}

impl<const D: usize> Segment<D> {
    pub fn new(start: Vector<D>, end: Vector<D>) -> Self {
        Self { start, end }
    }

    pub fn intersects_segment(&self, other: &Self) -> bool {
        todo!();
    }
}
