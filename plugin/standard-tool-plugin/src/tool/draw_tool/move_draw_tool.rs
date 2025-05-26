use crate::tool::{Interaction, Tool};
use crate::traits::AddEntity;
use crate::MoveDraw;
use core::entity::Entity;
use geometry::figure::point::Point;

mod render;

pub struct MoveDrawTool {
    start: Option<Point>,
    drawable: Option<Entity>,
    build_drawable: Box<dyn Fn() -> Entity>,
}

impl MoveDrawTool {
    pub fn new(build_drawable: impl Fn() -> Entity + 'static) -> MoveDrawTool {
        Self {
            start: None,
            drawable: None,
            build_drawable: Box::new(build_drawable),
        }
    }

    pub fn end_drawing(&mut self) {
        self.start.take();

        let Some(drawable) = self.drawable.take() else {
            return;
        };

        /* send event for element draw */
    }
}

impl Tool for MoveDrawTool {
    fn interaction_event(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => {
                let mut drawable: Entity = (self.build_drawable)();
                self.start.replace(position.clone());

                let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.mouse_down)(&mut drawable, &position);

                self.drawable = Some(drawable);
            }
            Interaction::PointerMove(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                let Some(start) = self.start else {
                    return;
                };

                let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.mouse_move)(drawable, &start, &position);
            }
            Interaction::PointerUp(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                /* Take value from start point to make sure after mouse up action start point is None */
                let Some(start) = self.start.take() else {
                    return;
                };

                let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.mouse_up)(drawable, &start, &position);

                self.end_drawing();
            }

            Interaction::KeyboardEvent(_) => {}
        }
    }
}
