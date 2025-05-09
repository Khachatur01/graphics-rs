use geometry::figure::point::Point;
use standard_rendering_plugin::renderer::Renderable;

pub mod select_tool;
pub mod draw_tool;


pub enum Key {
    Esc,
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
}

pub enum Interaction {
    PointerDown(Point, PointingDevice),
    PointerMove(Point, PointingDevice),
    PointerUp(Point, PointingDevice),

    KeyboardEvent(Key),
}

pub trait Tool: Renderable {
    fn interaction_event(&mut self, interaction: Interaction);
    // fn mouse_down(&mut self, point: &Point);
    //
    // fn mouse_move(&mut self, point: &Point);
    //
    // fn mouse_up(&mut self, point: &Point);
}
