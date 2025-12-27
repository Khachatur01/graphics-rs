use algebra::linear::vector::Vector;

#[derive(Clone)]
pub struct Ellipse {
    center: Vector<2>,
    x_radius: f64,
    y_radius: f64,
}

impl Ellipse {
    pub fn new(center: Vector<2>, x_radius: f64, y_radius: f64) -> Ellipse {
        Ellipse {
            center,
            x_radius,
            y_radius,
        }
    }
}
