use crate::figure::path::command::positioning::Positioning;
use crate::point::point_2d::Point2D;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, GetterMethods)]
pub struct BezierTo {
    control_points: Vec<Point2D>,
    reflected: bool,

    to_point: Point2D,
    positioning: Positioning,
}
