use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vector<const D: usize> {
    #[serde(with = "serde_arrays")]
    dimensions: [f64; D]
}

impl Vector<2> {
    pub fn new(x: f64, y: f64) -> Self {
        Self { dimensions: [x, y] }
    }

    pub fn x(&self) -> f64 {
        self.dimensions[0]
    }

    pub fn y(&self) -> f64 {
        self.dimensions[1]
    }
}

impl Vector<3> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { dimensions: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.dimensions[0]
    }

    pub fn y(&self) -> f64 {
        self.dimensions[1]
    }

    pub fn z(&self) -> f64 {
        self.dimensions[2]
    }
}

impl<const D: usize> Vector<D> {
    pub fn from_array(dimensions: [f64; D]) -> Self {
        Self { dimensions }
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.dimensions[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.dimensions[index]
    }
}
