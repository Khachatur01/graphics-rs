use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::draw_tool::move_draw_tool::event_channel::{EndDrawing, EventChannel, MouseDown, MouseMove, MouseUp};
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

pub mod event_channel;

pub struct MoveDrawTool<Drawable: Clone> {
    start: Option<Point>,
    drawable: Drawable,
    pub event_channel: EventChannel<Drawable>
}

impl<Drawable: Clone> MoveDrawTool<Drawable> {
    pub fn new(drawable: Drawable) -> MoveDrawTool<Drawable> {
        Self {
            start: None,
            drawable,
            event_channel: Default::default()
        }
    }

    pub fn end_drawing(&mut self) {
        self.start.take();

        let _ = self.event_channel.end_drawing.send(EndDrawing { drawable: self.drawable.clone() });
    }
}

impl<Drawable: MoveDraw + Clone> Interactive for MoveDrawTool<Drawable> {
    fn mouse_down(&mut self, point: &Point) {
        self.start.replace(point.clone());
        self.drawable.mouse_down(point);

        let _ = self.event_channel.mouse_down.send(MouseDown { drawable: self.drawable.clone(), point: point.clone() });
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(start) = self.start else { return; };
        self.drawable.mouse_move(&start, point);

        let _ = self.event_channel.mouse_move.send(MouseMove { drawable: self.drawable.clone(), point: point.clone() });
    }

    fn mouse_up(&mut self, point: &Point) {
        /* Take value from start point to make sure after mouse up action start point is None */
        let Some(start) = self.start.take() else { return; };
        self.drawable.mouse_up(&start, point);

        let _ = self.event_channel.mouse_up.send(MouseUp { drawable: self.drawable.clone(), point: point.clone() });

        self.end_drawing();
    }
}

impl<Drawable: MoveDraw + Clone> Tool for MoveDrawTool<Drawable> {}
