mod render;

use crate::interactivity::tool::Interactive;
use crate::interactivity::tool::Tool;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::Resize;

pub struct SelectTool<Id> {
    selected_elements: Vec<Id>,
    selection: Option<Rectangle>,
}

impl<Id> SelectTool<Id> {
    pub fn new() -> Self {
        Self {
            selected_elements: vec![],
            selection: None,
        }
    }
}

impl<Id> Interactive for SelectTool<Id> {
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

impl<Id> Tool for SelectTool<Id> {}