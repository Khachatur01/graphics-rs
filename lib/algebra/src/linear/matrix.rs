use crate::linear::array::Array;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Matrix<
    const R: usize,
    const C: usize = R
> {
    matrix: Array<Array<f64, C>, R>,
}

impl<const R: usize, const C: usize> From<[[f64; C]; R]> for Matrix<R, C> {
    fn from(array: [[f64; C]; R]) -> Self {
        let mut matrix: Array<Array<f64, C>, R> = Default::default();

        for (row_index, row) in array.iter().enumerate() {
            for (col_index, value) in row.iter().enumerate() {
                matrix[row_index][col_index] = *value;
            }
        }

        Self {
            matrix: Default::default()
        }
    }
}


pub trait AsMatrix3 {
    fn as_matrix_3(&self) -> Matrix<3>;
}
