mod render;

use crate::tool::{Interaction, Tool};
use entity_model_feature::entity_id::EntityId;
use geometry::figure::rectangle::Rectangle;
use geometry::point::point_2d::Point2D;

pub struct SelectTool<Id: EntityId> {
    selected_elements: Vec<Id>,
    selection: Option<Rectangle>,
}

impl<Id: EntityId> SelectTool<Id> {
    pub fn new() -> Self {
        Self {
            selected_elements: vec![],
            selection: None,
        }
    }
}

impl<Id: EntityId> Tool for SelectTool<Id> {
    fn interact(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => {
                self.selection = Some(Rectangle::zero_sized(position));
            }
            Interaction::PointerMove(position, _) => {
                let Some(selection) = &mut self.selection else {
                    return;
                };

                let delta: Point2D = position - selection.top_left();
                selection.set_size(delta.x(), delta.y());

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
