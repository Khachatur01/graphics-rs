use crate::figure::path::command::positioning::Positioning;
use crate::point::point_2d::Point2D;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, GetterMethods)]
pub struct MoveTo {
    pub to_point: Point2D,
    pub positioning: Positioning,
}
