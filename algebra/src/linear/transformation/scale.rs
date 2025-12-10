use crate::linear::matrix::{AsMatrix3, Matrix};
use crate::linear::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Scale<const N: usize> {
    by: Vector<3>,
    reference_point: Vector<N>,
}

impl AsMatrix3 for Scale<2> {
    fn as_matrix_3(&self) -> Matrix<3> {
        todo!()
    }
}

impl AsMatrix3 for Scale<3> {
    fn as_matrix_3(&self) -> Matrix<3> {
        todo!()
    }
}
