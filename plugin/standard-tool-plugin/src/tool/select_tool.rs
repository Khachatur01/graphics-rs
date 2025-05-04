mod render;

use core::interactivity::Interactive;
use core::entity::Id;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::Resize;
use crate::tool::Tool;

pub struct SelectTool {
    selected_elements: Vec<Box<dyn Id>>,
    selection: Option<Rectangle>,
}

impl SelectTool {
    pub fn new() -> Self {
        Self {
            selected_elements: vec![],
            selection: None,
        }
    }
}

impl Interactive for SelectTool {
    fn mouse_down(&mut self, point: &Point) {
        self.selection = Some(
            Rectangle::zero_sized(point.clone())
        );
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(selection) = &mut self.selection else {
            return;
        };

        let delta: Point = point - selection.top_left();
        selection.resize(delta.x(), delta.y());
    }

    fn mouse_up(&mut self, _: &Point) {
        self.selection = None;
    }
}

impl Tool for SelectTool {}
