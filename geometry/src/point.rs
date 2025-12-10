use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub mod point_2d;
pub mod point_3d;

pub trait Point:
    Default +
    Copy + Clone +
    Add + Sub + AddAssign + SubAssign +
    Serialize + Deserialize<'static> {}


pub struct Point1<const N: usize> {
    coordinates: [f64; N]
}

impl<const N: usize> Add for &Point1<N> {
    type Output = Point1<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Point1 { coordinates: [0.0; N] };
        for i in 0..N {
            result.coordinates[i] = self.coordinates[i] + rhs.coordinates[i];
        }
        result
    }
}

impl<const N: usize> Add for Point1<N> {
    type Output = Point1<N>;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<const N: usize> Sub for &Point1<N> {
    type Output = Point1<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Point1 { coordinates: [0.0; N] };
        for i in 0..N {
            result.coordinates[i] = self.coordinates[i] - rhs.coordinates[i];
        }
        result
    }
}

impl<const N: usize> Sub for Point1<N> {
    type Output = Point1<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}
