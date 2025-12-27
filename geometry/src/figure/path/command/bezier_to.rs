use crate::figure::path::command::positioning::Positioning;
use algebra::linear::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BezierTo {
    control_points: Vec<Vector<2>>,
    reflected: bool,

    to_point: Vector<2>,
    positioning: Positioning,
}
