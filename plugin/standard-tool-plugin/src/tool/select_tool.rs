mod render;

use entity_model_feature::EntityId;
use crate::tool::{Interaction, Tool};
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::Resize;

pub struct SelectTool {
    selected_elements: Vec<Box<dyn EntityId>>,
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

impl Tool for SelectTool {
    fn interact(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => {
                self.selection = Some(Rectangle::zero_sized(position));
            }
            Interaction::PointerMove(position, _) => {
                let Some(selection) = &mut self.selection else {
                    return;
                };

                let delta: Point = position - selection.top_left();
                selection.resize(delta.x(), delta.y());

                self.selected_elements.clear();
            }
            Interaction::PointerUp(_, _) => {
                self.selection = None;
            }
            Interaction::KeyDown(_) => {}
            Interaction::KeyUp(_) => {}
        }
    }
}
