use getter_methods::GetterMethods;
use impl_ops::impl_op_ex;
use serde::{Deserialize, Serialize};
use std::ops;
use std::ops::{AddAssign, SubAssign};

#[derive(GetterMethods, Serialize, Deserialize, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new_2d(x: f64, y: f64) -> Point {
        Point { x, y, z: 0.0 }
    }
    pub fn new_3d(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y, z: 0.0 }
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl_op_ex!(+ |lhs: &Point, rhs: &Point| -> Point {
    Point {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
});

impl_op_ex!(-|lhs: &Point, rhs: &Point| -> Point {
    Point {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
