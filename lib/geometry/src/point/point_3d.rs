use crate::point::Point;
use getter_methods::GetterMethods;
use impl_ops::impl_op_ex;
use serde::{Deserialize, Serialize};
use std::ops;
use std::ops::{AddAssign, SubAssign};

#[derive(GetterMethods, Serialize, Deserialize, Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
}

impl From<(f64, f64, f64)> for Point3D {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl Default for Point3D {
    fn default() -> Self {
        Point3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl_op_ex!(+ |lhs: &Point3D, rhs: &Point3D| -> Point3D {
    Point3D {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
});

impl_op_ex!(-|lhs: &Point3D, rhs: &Point3D| -> Point3D {
    Point3D {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});

impl AddAssign<&Point3D> for Point3D {
    fn add_assign(&mut self, rhs: &Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl AddAssign<Point3D> for Point3D {
    fn add_assign(&mut self, rhs: Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<&Point3D> for Point3D {
    fn sub_assign(&mut self, rhs: &Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl SubAssign<Point3D> for Point3D {
    fn sub_assign(&mut self, rhs: Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Point for Point3D {}
