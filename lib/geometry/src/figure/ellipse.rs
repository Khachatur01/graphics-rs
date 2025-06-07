use crate::point::point_2d::Point2D;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Ellipse {
    center: Point2D,
    x_radius: f64,
    y_radius: f64,
}

impl Ellipse {
    pub fn new(center: Point2D, x_radius: f64, y_radius: f64) -> Ellipse {
        Ellipse {
            center,
            x_radius,
            y_radius,
        }
    }
}
