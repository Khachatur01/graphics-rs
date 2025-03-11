use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::Tool;
use crate::Interactive;
use crossbeam_channel::{unbounded, Sender};
use geometry::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct EventHandler<Drawable> {
    end_drawing: Sender<Drawable>,
    mouse_down: Sender<(Drawable, Point)>,
    mouse_move: Sender<(Drawable, Point)>,
    mouse_up: Sender<(Drawable, Point)>,
}

impl <Drawable> Default for EventHandler<Drawable> {
    fn default() -> Self {
        let (end_drawing, _) = unbounded();
        let (mouse_down, _) = unbounded();
        let (mouse_move, _) = unbounded();
        let (mouse_up, _) = unbounded();

        Self {
            end_drawing,
            mouse_down,
            mouse_move,
            mouse_up,
        }
    }
}

pub struct MoveDrawTool<Drawable> {
    start: Option<Point>,
    drawable: Drawable,
    event_handler: EventHandler<Drawable>,
}

impl<Drawable: Copy + Clone> MoveDrawTool<Drawable> {
    pub fn new(drawable: Drawable) -> MoveDrawTool<Drawable> {
        Self {
            start: None,
            drawable,
            event_handler: Default::default(),
        }
    }

    pub fn event_handler(&self) -> &EventHandler<Drawable> {
        &self.event_handler
    }

    pub fn end_drawing(&mut self) {
        self.start.take();

        let _ = self.event_handler.end_drawing.send(self.drawable);
    }
}

impl<Drawable: MoveDraw + Copy + Clone> Interactive for MoveDrawTool<Drawable> {
    fn mouse_down(&mut self, point: &Point) {
        self.start.replace(point.clone());
        self.drawable.mouse_down(point);

        let _ = self.event_handler.mouse_down.send((self.drawable, *point));
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(start) = self.start else { return; };
        self.drawable.mouse_move(&start, point);

        let _ = self.event_handler.mouse_move.send((self.drawable, *point));
    }

    fn mouse_up(&mut self, point: &Point) {
        /* Take value from start point to make sure after mouse up action start point is None */
        let Some(start) = self.start.take() else { return; };
        self.drawable.mouse_up(&start, point);

        let _ = self.event_handler.mouse_up.send((self.drawable, *point));

        self.end_drawing();
    }
}

impl<Drawable: MoveDraw + Copy + Clone> Tool for MoveDrawTool<Drawable> {}
