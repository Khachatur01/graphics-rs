use crate::figure::path::command::positioning::Positioning;
use crate::point::point_2d::Point2D;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct MoveTo {
    to_point: Point2D,
    positioning: Positioning,
}
