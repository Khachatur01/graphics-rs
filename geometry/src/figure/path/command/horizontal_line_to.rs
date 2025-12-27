use crate::figure::path::command::positioning::Positioning;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HorizontalLineTo {
    x: f64,
    positioning: Positioning,
}
