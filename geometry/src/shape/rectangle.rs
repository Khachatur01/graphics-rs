use crate::shape::point::Point;
use crate::shape::polygon::Polygon;
use crate::traits::{Drag, Resize};
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(top_left: Point, width: f64, height: f64) -> Rectangle {
        Rectangle { top_left, width, height }
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

impl Resize for Rectangle {
    fn resize(&mut self, new_width: f64, new_height: f64) {
        self.width = new_width;
        self.height = new_height;
    }
}

impl Drag for Rectangle {
    fn drag(&mut self, delta: Point) {
        self.top_left += delta;
    }
}
