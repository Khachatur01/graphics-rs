use getter_methods::GetterMethods;
use impl_ops::impl_op_ex;
use std::ops;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(GetterMethods, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

impl_op_ex!(+ |lhs: &Point, rhs: &Point| -> Point {
    Point {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y
    }
});

impl_op_ex!(- |lhs: &Point, rhs: &Point| -> Point {
    Point {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y
    }
});

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
