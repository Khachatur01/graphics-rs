mod render;
mod tool;

use crate::tool::draw_tool::draw_mode::ClickDraw;
use geometry::figure::point::Point;

pub struct ClickDrawTool<Drawable: ClickDraw> {
    points: Vec<Point>,
    drawable: Drawable
}

impl<Drawable: ClickDraw> ClickDrawTool<Drawable> {
    pub fn new(drawable: Drawable) -> ClickDrawTool<Drawable> {
        Self { points: Vec::new(), drawable }
    }

    pub fn end_drawing(&mut self) {
        self.points.clear();

        /* todo: add copy of drawn element_view to viewport */
    }
}
