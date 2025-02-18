use crate::figure::point::Point;
use crate::math::{Drag, Resize};
use getter_methods::GetterMethods;

#[derive(GetterMethods, Copy, Clone)]
pub struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(top_left: Point, width: f64, height: f64) -> Rectangle {
        Rectangle { top_left, width, height }
    }

    pub fn zero_sized(top_left: Point) -> Rectangle {
        Rectangle { top_left, width: 0.0, height: 0.0 }
    }
}


impl Resize for Rectangle {
    fn resize(&mut self, new_width: f64, new_height: f64) {
        self.width = new_width;
        self.height = new_height;
    }
}

impl Drag for Rectangle {
    fn drag(&mut self, delta: &Point) {
        self.top_left += delta;
    }
}
