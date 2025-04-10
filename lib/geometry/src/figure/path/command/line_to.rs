use crate::figure::path::command::positioning::Positioning;
use crate::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct LineTo {
    to_point: Point,
    positioning: Positioning,
}
