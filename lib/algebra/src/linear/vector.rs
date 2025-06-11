use crate::linear::array::Array;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vector<const N: usize> {
    vector: Array<f64, N>
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}
