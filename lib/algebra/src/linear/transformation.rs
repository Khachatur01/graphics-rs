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
    Matrix3D(Matrix<3>),

    Translation2D(Translation<2>),
    Rotation2D(Rotation<2>),
    Skew2D(Skew<2>),
    Scale2D(Scale<2>),

    Translation3D(Translation<3>),
    Rotation3D(Rotation<3>),
    Skew3D(Skew<3>),
    Scale3D(Scale<3>),
}
