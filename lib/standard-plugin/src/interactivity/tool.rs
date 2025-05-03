use core::interactivity::Interactive;
use rendering::Renderable;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: Interactive + Renderable {}
