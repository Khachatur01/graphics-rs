use geometry::point::point_2d::Point2D;
use standard_rendering_plugin::renderable::Renderable;

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
    PointerDown(Point2D, PointingDevice),
    PointerMove(Point2D, PointingDevice),
    PointerUp(Point2D, PointingDevice),

    KeyDown(Key),
    KeyUp(Key),
}

pub trait Tool: Renderable {
    fn interact(&mut self, interaction: Interaction);
}
