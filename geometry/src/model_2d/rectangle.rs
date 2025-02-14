use getter_methods::GetterMethods;
use crate::model_2d::point::Point;
use crate::model_2d::polygon::Polygon;

#[derive(GetterMethods)]
pub struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(top_left: Point, width: f64, height: f64) -> Rectangle {
        Rectangle {top_left, width, height}
    }
}

impl Into<Polygon> for Rectangle {
    fn into(self) -> Polygon {
        Polygon::new(vec![
            /* top left */
            Point::new(self.top_left.x(),                     self.top_left.y()),
            /* top right */
            Point::new(self.top_left.x() + self.width,        self.top_left.y()),
            /* bottom right */
            Point::new(self.top_left.x() + self.width,        self.top_left.y() + self.height),
            /* bottom left */
            Point::new(self.top_left.x(),                     self.top_left.y() + self.height),
        ])
    }
}
