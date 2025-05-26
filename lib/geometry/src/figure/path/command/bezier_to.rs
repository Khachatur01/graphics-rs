use crate::figure::path::command::positioning::Positioning;
use crate::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct BezierTo {
    control_points: Vec<Point>,
    reflected: bool,

    to_point: Point,
    positioning: Positioning,
}
