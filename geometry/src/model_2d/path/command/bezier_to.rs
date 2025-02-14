use crate::model_2d::path::command::positioning::Positioning;
use crate::model_2d::point::Point;

pub struct BezierTo {
    control_points: Vec<Point>,
    reflected: bool,

    to_point: Point,
    positioning: Positioning
}
