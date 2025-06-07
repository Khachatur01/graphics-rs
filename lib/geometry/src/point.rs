use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub mod point_2d;
pub mod point_3d;

pub trait Point:
    Default +
    Copy + Clone +
    Add + Sub + AddAssign + SubAssign +
    Serialize + Deserialize<'static> {}
