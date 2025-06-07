use crate::figure::path::command::positioning::Positioning;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, GetterMethods)]
pub struct VerticalLineTo {
    y: f64,
    positioning: Positioning,
}
