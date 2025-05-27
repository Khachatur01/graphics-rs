mod render;

use crate::tool::{Interaction, Tool};
use crate::traits::AddEntity;
use crate::ClickDraw;
use core::entity::Entity;

pub struct ClickDrawTool {
    drawable: Option<Entity>,
    build_drawable: Box<dyn Fn() -> Entity>,
}

impl ClickDrawTool {
    pub fn new(build_drawable: impl Fn() -> Entity + 'static) -> ClickDrawTool {
        Self {
            build_drawable: Box::new(build_drawable),
            drawable: None,
        }
    }

    pub fn end_drawing(&mut self) {
        let Some(drawable) = self.drawable.take() else {
            return;
        };

        /* send event for element draw */
    }
}

impl Tool for ClickDrawTool {
    fn interaction_event(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => match &mut self.drawable {
                None => {
                    let mut drawable: Entity = (self.build_drawable)();

                    let click_draw: &ClickDraw =
                        drawable.query().expect("Failed to query ClickDraw");
                    (click_draw.mouse_down)(&mut drawable, &position);

                    self.drawable = Some(drawable);
                }
                Some(drawable) => {
                    let click_draw: &ClickDraw =
                        drawable.query().expect("Failed to query ClickDraw");
                    (click_draw.mouse_down)(drawable, &position);
                }
            },
            Interaction::PointerMove(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
                (click_draw.mouse_move)(drawable, &position);
            }
            Interaction::PointerUp(_, _) => {}

            Interaction::KeyboardEvent(_) => {}
        }
    }
}
