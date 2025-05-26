use crate::figure::path::command::positioning::Positioning;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct VerticalLineTo {
    y: f64,
    positioning: Positioning,
}
