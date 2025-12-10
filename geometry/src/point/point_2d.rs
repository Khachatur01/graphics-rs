use crate::point::Point;
use getter_methods::GetterMethods;
use impl_ops::impl_op_ex;
use serde::{Deserialize, Serialize};
use std::ops;
use std::ops::{AddAssign, SubAssign};

#[derive(GetterMethods, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }
}

impl From<(f64, f64)> for Point2D {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl Default for Point2D {
    fn default() -> Self {
        Point2D { x: 0.0, y: 0.0 }
    }
}

impl_op_ex!(+ |lhs: &Point2D, rhs: &Point2D| -> Point2D {
    Point2D {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
    }
});

impl_op_ex!(-|lhs: &Point2D, rhs: &Point2D| -> Point2D {
    Point2D {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
    }
});

impl AddAssign<&Point2D> for Point2D {
    fn add_assign(&mut self, rhs: &Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<&Point2D> for Point2D {
    fn sub_assign(&mut self, rhs: &Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl SubAssign<Point2D> for Point2D {
    fn sub_assign(&mut self, rhs: Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Point for Point2D {}
