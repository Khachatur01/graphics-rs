use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::draw_tool::move_draw_tool::event_channel::{EndDrawing, EventChannel};
use geometry::figure::point::Point;

pub mod event_channel;
mod render;
mod tool;

pub struct MoveDrawTool<Drawable: MoveDraw> {
    start: Option<Point>,
    drawable: Option<Drawable>,
    pub event_channel: EventChannel<Drawable>
}

impl<Drawable: MoveDraw> MoveDrawTool<Drawable> {
    pub fn new() -> MoveDrawTool<Drawable> {
        Self {
            start: None,
            drawable: None,
            event_channel: Default::default()
        }
    }

    pub fn end_drawing(&mut self) {
        let Some(drawable) = &self.drawable else {
            return;
        };

        self.start.take();

        let _ = self.event_channel.end_drawing.send(EndDrawing { drawable: drawable.clone() });
    }
}
