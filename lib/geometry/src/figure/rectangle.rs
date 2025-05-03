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
    pub fn new(top_left: Point, width: f64, height: f64) -> Self {
        Self { top_left, width, height }
    }

    pub fn zero_sized(top_left: Point) -> Self {
        Self { top_left, width: 0.0, height: 0.0 }
    }

    pub fn absolute_sized(&self) -> Self {
        let mut rectangle_clone: Rectangle = self.clone();

        if rectangle_clone.width < 0.0 {
            rectangle_clone.width *= -1.0;
            rectangle_clone.top_left.x -= rectangle_clone.width;
        }
        if rectangle_clone.height < 0.0 {
            rectangle_clone.height *= -1.0;
            rectangle_clone.top_left.y -= rectangle_clone.height;
        }

        rectangle_clone
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
