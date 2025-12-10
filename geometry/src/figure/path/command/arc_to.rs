use crate::figure::path::command::positioning::Positioning;
use crate::point::point_2d::Point2D;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, GetterMethods)]
pub struct ArcTo {
    rx: f64,
    ry: f64,

    x_axis_rotation: u8,
    large_arc_flag: bool,
    sweep_flag: bool,

    to_point: Point2D,
    positioning: Positioning,
}
