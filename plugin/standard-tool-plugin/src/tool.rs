use geometry::figure::point::Point;
use standard_rendering_plugin::renderer::Renderable;

pub mod draw_tool;
pub mod select_tool;

pub enum Key {
    Enter,
    Backspace,
    Delete,
    Esc,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Ctrl,
    Alt,
    Shift,
    CapsLock,
    Fn,
    Win,
    Letter(char),
}

pub enum PointingDevice {
    Mouse,
    TouchPad,
    Unknown,
}

pub enum Interaction {
    PointerDown(Point, PointingDevice),
    PointerMove(Point, PointingDevice),
    PointerUp(Point, PointingDevice),

    KeyDown(Key),
    KeyUp(Key),
}

pub trait Tool: Renderable {
    fn interact(&mut self, interaction: Interaction);
}
