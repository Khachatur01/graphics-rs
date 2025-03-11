use crate::tool::draw_tool::draw_mode::ClickDraw;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

pub struct ClickDrawTool<Drawable> {
    points: Vec<Point>,
    drawable: Drawable
}

impl<Drawable> ClickDrawTool<Drawable> {
    pub fn new(drawable: Drawable) -> ClickDrawTool<Drawable> {
        Self { points: Vec::new(), drawable }
    }

    pub fn end_drawing(&mut self) {
        self.points.clear();

        /* todo: add copy of drawn element to viewport */
    }
}

impl<Drawable: ClickDraw> Interactive for ClickDrawTool<Drawable> {
    fn mouse_down(&mut self, point: &Point) {
        todo!()
    }

    fn mouse_move(&mut self, point: &Point) {
        todo!()
    }

    fn mouse_up(&mut self, point: &Point) {
        todo!()
    }
}

impl<Drawable: ClickDraw> Tool for ClickDrawTool<Drawable> {}
