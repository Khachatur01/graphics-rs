use getter_methods::GetterMethods;
use crate::model_2d::point::Point;

#[derive(GetterMethods)]
pub struct Ellipse {
    center: Point,
    x_radius: f64,
    y_radius: f64,
}

impl Ellipse {
    pub fn new(center: Point, x_radius: f64, y_radius: f64) -> Ellipse {
        Ellipse { center, x_radius, y_radius }
    }
}
