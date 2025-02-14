use getter_methods::GetterMethods;
use crate::shape::path::command::positioning::Positioning;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct ArcTo {
    rx: f64,
    ry: f64,

    x_axis_rotation: u8,
    large_arc_flag: bool,
    sweep_flag: bool,

    to_point: Point,
    positioning: Positioning,
}
