use algebra::linear::vector::Vector;

#[derive(Clone)]
pub struct Circle {
    center: Vector<2>,
    radius: f64,
}

impl Circle {
    pub fn new(center: Vector<2>, radius: f64) -> Circle {
        Circle { center, radius }
    }
}
