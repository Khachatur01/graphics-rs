use getter_methods::GetterMethods;
use crate::model_2d::path::command::positioning::Positioning;
use crate::model_2d::point::Point;

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
