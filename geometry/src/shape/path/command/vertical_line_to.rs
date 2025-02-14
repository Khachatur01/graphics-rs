use getter_methods::GetterMethods;
use crate::shape::path::command::positioning::Positioning;

#[derive(GetterMethods)]
pub struct VerticalLineTo {
    y: f64,
    positioning: Positioning
}
