use crate::figure::path::command::positioning::Positioning;
use algebra::linear::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ArcTo {
    radius: Vector<2>,

    x_axis_rotation: u8,
    large_arc_flag: bool,
    sweep_flag: bool,

    to_point: Vector<2>,
    positioning: Positioning,
}
