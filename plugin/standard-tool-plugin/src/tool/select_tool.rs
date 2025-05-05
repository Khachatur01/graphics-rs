mod render;

use core::interactivity::Interactive;
use core::entity::Id;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::Resize;
use crate::tool::Tool;
use crate::traits::{AddEntity, GetEntities};

pub struct SelectTool {
    selected_elements: Vec<Box<dyn Id>>,
    selection: Option<Rectangle>,
    view_port: Box<dyn GetEntities>,
}

impl SelectTool {
    pub fn new(view_port: impl GetEntities + 'static) -> Self {
        Self {
            selected_elements: vec![],
            selection: None,
            view_port: Box::new(view_port),
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

        self.selected_elements.clear();

        // self.view_port
        //     .get_entities()
        //     .iter()
        //     .filter_map(|entity| {
        //         if let Some(select) = entity.query::<Select>() {
        //             if (select.select)(entity, selection) {
        //                 return Some(entity);
        //             } else {
        //                 return None;
        //             }
        //         } else {
        //             return None;
        //         }
        //     })
        //     .for_each(|entity| {
        //         self.selected_elements.push(Box::new(entity.id()));
        //     });
    }

    fn mouse_up(&mut self, _: &Point) {
        self.selection = None;
    }
}

impl Tool for SelectTool {}
