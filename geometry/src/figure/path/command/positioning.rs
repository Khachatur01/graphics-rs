use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Positioning {
    Absolute,
    Relative,
}
