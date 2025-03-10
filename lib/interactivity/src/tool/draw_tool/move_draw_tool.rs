use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

pub struct MoveDrawTool<Drawable> {
    start: Option<Point>,
    drawable: Drawable,
}

impl<Drawable> MoveDrawTool<Drawable> {
    pub fn new(start: Option<Point>, drawable: Drawable) -> MoveDrawTool<Drawable> {
        Self { start, drawable }
    }

    pub fn end_drawing(&mut self) {
        self.start.take();

        /* todo: add copy of drawn element to viewport */
    }
}

impl<Drawable: MoveDraw> Interactive for MoveDrawTool<Drawable> {
    fn mouse_down(&mut self, point: &Point) {
        self.start.replace(point.clone());
        self.drawable.mouse_down(point);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(start) = self.start else { return; };
        self.drawable.mouse_move(&start, point);
    }

    fn mouse_up(&mut self, point: &Point) {
        /* Take value from start point to make sure after mouse up action start point is None */
        let Some(start) = self.start.take() else { return; };
        self.drawable.mouse_up(&start, point);

        self.end_drawing();
    }
}

impl<Drawable: MoveDraw> Tool for MoveDrawTool<Drawable> {}
