use crate::linear::matrix::{AsMatrix3, Matrix};
use crate::linear::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Skew<const N: usize> {
    angle: Vector<N>,
    reference_point: Vector<N>,
}

impl AsMatrix3 for Skew<2> {
    fn as_matrix_3(&self) -> Matrix<3> {
        todo!()
    }
}

impl AsMatrix3 for Skew<3> {
    fn as_matrix_3(&self) -> Matrix<3> {
        todo!()
    }
}
