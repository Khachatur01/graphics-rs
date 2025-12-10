use crate::linear::matrix::Matrix;
use crate::linear::transformation::rotation::Rotation;
use crate::linear::transformation::scale::Scale;
use crate::linear::transformation::skew::Skew;
use crate::linear::transformation::translation::Translation;
use serde::{Deserialize, Serialize};

pub mod translation;
pub mod rotation;
pub mod skew;
pub mod scale;


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Transformation {
    Matrix3(Matrix<3>),

    Translation2(Translation<2>),
    Rotation2(Rotation<2>),
    Skew2(Skew<2>),
    Scale2(Scale<2>),

    Translation3(Translation<3>),
    Rotation3(Rotation<3>),
    Skew3(Skew<3>),
    Scale3(Scale<3>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transformations(Vec<Transformation>);

impl Transformations {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, transformation: Transformation) {
        self.0.push(transformation);
    }

    pub fn compose(&self) -> Option<Matrix<3>> {
        let matrix = Matrix::from(
            [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ]
        );

        Some(matrix)
    }
}
