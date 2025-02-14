use getter_methods::GetterMethods;
use crate::shape::path::command::positioning::Positioning;

#[derive(GetterMethods)]
pub struct HorizontalLineTo {
    x: f64,
    positioning: Positioning
}
