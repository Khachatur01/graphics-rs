pub mod draw_mode;

use crate::tool::draw_tool::draw_mode::DrawMode;
use crate::tool::Tool;
use crate::MouseEvents;
use geometry::figure::point::Point;

pub struct DrawTool {
    draw_mode: DrawMode,
}

impl DrawTool {
    pub fn new(draw_mode: DrawMode) -> Self {
        Self {
            draw_mode,
        }
    }

    pub fn end_drawing(&mut self) {
        match &mut self.draw_mode {
            DrawMode::Move { start, .. } => {
                start.take();
            }
            DrawMode::Click { points, .. } => {
                points.clear();
            }
        };

        /* todo: add copy of drawn element to viewport */
    }
}

impl MouseEvents for DrawTool {
    fn mouse_down(&mut self, point: &Point) {
        match &mut self.draw_mode {
            DrawMode::Move { start, drawable } => {
                start.replace(point.clone());
                drawable.mouse_down(point);
            }
            DrawMode::Click { points, drawable } => {
                todo!()
            }
        }
    }

    fn mouse_move(&mut self, point: &Point) {
        match &mut self.draw_mode {
            DrawMode::Move { start, drawable } => {
                let Some(start) = start else { return; };
                drawable.mouse_move(start, point);
            }
            DrawMode::Click { points, drawable } => {
                todo!()
            }
        }
    }

    fn mouse_up(&mut self, point: &Point) {
        match &mut self.draw_mode {
            DrawMode::Move { start, drawable } => {
                /* Take value from start point to make sure after mouse up action start point is None */
                let Some(start) = start.take() else { return; };
                drawable.mouse_up(&start, point);

                self.end_drawing();
            }
            DrawMode::Click { points, drawable } => {
                todo!()
            }
        }
    }
}

impl Tool for DrawTool {}
