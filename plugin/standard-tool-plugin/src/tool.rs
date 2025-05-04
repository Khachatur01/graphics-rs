use core::interactivity::Interactive;
use standard_rendering_plugin::renderer::Renderable;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: Interactive + Renderable {}
