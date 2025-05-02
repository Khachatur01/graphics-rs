use crate::Interactive;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: Interactive {}
