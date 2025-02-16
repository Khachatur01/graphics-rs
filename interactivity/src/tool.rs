use view_port::view_port::traits::MouseEvents;

pub mod move_draw_tool;
pub mod click_draw_tool;
pub mod select_tool;

pub trait Tool: MouseEvents {}
