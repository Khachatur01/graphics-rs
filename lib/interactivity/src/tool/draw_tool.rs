pub mod draw;

use crate::tool::draw_tool::draw::Draw;
use crate::tool::Tool;
use crate::MouseEvents;
use geometry::figure::point::Point;

pub struct DrawTool<Drawable>
where Drawable: Draw {
    points: Vec<Point>,
    drawable: Drawable,
}

impl<Drawable> DrawTool<Drawable>
where Drawable: Draw {
    pub fn new(drawable: Drawable) -> Self {
        Self {
            points: vec![],
            drawable,
        }
    }

    pub fn end_drawing(&mut self) {
        self.points.clear();
        /* todo: add copy of drawn element to viewport */
    }
}

impl<Drawable> MouseEvents for DrawTool<Drawable>
where Drawable: Draw {
    fn mouse_down(&mut self, point: &Point) {
        self.points.push(*point);

        self.drawable.mouse_down(&self.points, point);
    }

    fn mouse_move(&mut self, point: &Point) {
        self.drawable.mouse_down(&self.points, point);
    }

    fn mouse_up(&mut self, point: &Point) {
        self.drawable.mouse_up(&self.points, point);
    }
}

impl<Drawable> Tool for DrawTool<Drawable>
where Drawable: Draw {}
