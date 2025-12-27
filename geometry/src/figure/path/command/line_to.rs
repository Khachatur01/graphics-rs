use crate::figure::path::command::positioning::Positioning;
use algebra::linear::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LineTo {
    pub to_point: Vector<2>,
    pub positioning: Positioning,
}
