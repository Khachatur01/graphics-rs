use core::interactivity::Interactive;
use plugin_rendering::Renderable;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: Interactive + Renderable {}
