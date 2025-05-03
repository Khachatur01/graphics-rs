mod render;

use crate::interactivity::tool::Tool;
use core::entity::model::Model;
use core::entity::Entity;
use core::interactivity::Interactive;
use geometry::figure::point::Point;
use rendering::{Renderable, Renderer};

pub struct ClickDrawTool {
    points: Vec<Point>,
    drawable: Option<Entity>,
}

impl Model for ClickDrawTool {}

impl ClickDrawTool {
    pub fn new() -> ClickDrawTool {
        Self { points: Vec::new(), drawable: None }
    }

    pub fn end_drawing(&mut self) {
        self.points.clear();

        /* todo: add copy of drawn element_view to viewport */
    }
}

impl Interactive for ClickDrawTool {
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

impl Tool for ClickDrawTool {}
