use view_port::view_port::traits::MouseEvents;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: MouseEvents {}
