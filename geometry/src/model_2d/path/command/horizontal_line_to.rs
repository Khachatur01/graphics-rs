use getter_methods::GetterMethods;
use crate::model_2d::path::command::positioning::Positioning;

#[derive(GetterMethods)]
pub struct HorizontalLineTo {
    x: f64,
    positioning: Positioning
}
