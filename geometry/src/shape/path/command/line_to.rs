use getter_methods::GetterMethods;
use crate::shape::path::command::positioning::Positioning;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct LineTo {
    to_point: Point,
    positioning: Positioning,
}
