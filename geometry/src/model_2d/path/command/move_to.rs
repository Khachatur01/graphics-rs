use getter_methods::GetterMethods;
use crate::model_2d::path::command::positioning::Positioning;
use crate::model_2d::point::Point;

#[derive(GetterMethods)]
pub struct MoveTo {
    to_point: Point,
    positioning: Positioning,
}
