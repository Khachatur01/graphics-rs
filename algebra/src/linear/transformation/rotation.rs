use crate::linear::matrix::{AsMatrix3, Matrix};
use crate::linear::vector::Vector;
use serde::{Deserialize, Serialize};

/* fixme */
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Rotation<const N: usize> {
    angle: f64,
    reference_point: Vector<N>,
}

impl AsMatrix3 for Rotation<2> {
    fn as_matrix_3(&self) -> Matrix<3> {
        todo!()
    }
}

impl AsMatrix3 for Rotation<3> {
    fn as_matrix_3(&self) -> Matrix<3> {
        todo!()
    }
}
