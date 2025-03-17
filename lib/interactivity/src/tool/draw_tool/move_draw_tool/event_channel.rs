use crate::channel::Channel;
use geometry::figure::point::Point;

#[derive(Clone)]
pub struct EndDrawing<Drawable: Clone> {
    pub drawable: Drawable
}
#[derive(Clone)]
pub struct MouseDown<Drawable: Clone> {
    pub drawable: Drawable,
    pub point: Point,
}
#[derive(Clone)]
pub struct MouseMove<Drawable: Clone> {
    pub drawable: Drawable,
    pub point: Point,
}
#[derive(Clone)]
pub struct MouseUp<Drawable: Clone> {
    pub drawable: Drawable,
    pub point: Point,
}

pub struct EventChannel<Drawable: Clone> {
    pub end_drawing: Channel<EndDrawing<Drawable>>,
    pub mouse_down: Channel<MouseDown<Drawable>>,
    pub mouse_move: Channel<MouseMove<Drawable>>,
    pub mouse_up: Channel<MouseUp<Drawable>>,
}

impl<Drawable: Clone> Default for EventChannel<Drawable> {
    fn default() -> Self {
        Self {
            end_drawing: Default::default(),
            mouse_down: Default::default(),
            mouse_move: Default::default(),
            mouse_up: Default::default(),
        }
    }
}
