use getter_methods::GetterMethods;
use crate::shape::path::command::positioning::Positioning;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct BezierTo {
    control_points: Vec<Point>,
    reflected: bool,

    to_point: Point,
    positioning: Positioning
}
