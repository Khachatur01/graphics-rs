use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub trait EntityId: Display + Serialize + Deserialize<'static> + Clone + 'static {}
