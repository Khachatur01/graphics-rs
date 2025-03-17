use crate::Interactive;
use rendering::Render;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: Interactive + Render {}
