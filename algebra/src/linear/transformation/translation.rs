use crate::linear::matrix::{AsMatrix3, Matrix};
use crate::linear::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Translation<const N: usize> {
    delta: Vector<N>,
}

impl AsMatrix3 for Translation<2> {
    fn as_matrix_3(&self) -> Matrix<3> {
        Matrix::from(
            [
                [0.0, 0.0, self.delta[0]],
                [0.0, 0.0, self.delta[1]],
                [0.0, 0.0, 0.0],
            ]
        )
    }
}

impl AsMatrix3 for Translation<3> {
    fn as_matrix_3(&self) -> Matrix<3> {
        Matrix::from(
            [
                [0.0, 0.0, self.delta[0]],
                [0.0, 0.0, self.delta[1]],
                [0.0, 0.0, self.delta[2]],
            ]
        )
    }
}
