mod render;

use entity_model_feature::entity::Entity;
use event_handler::EventHandler;
use geometry::figure::point::Point;
use crate::tool::{Interaction, Key, Tool};
use crate::ClickDraw;

#[derive(Default)]
pub struct Hook {
    pub pointer_down: EventHandler<Point>,
    pub pointer_move: EventHandler<Point>,
    pub pointer_up: EventHandler<Point>,
    pub end_drawing: EventHandler<Entity>
}

pub struct ClickDrawTool {
    pub hook: Hook,

    drawable: Option<Entity>,
    build_drawable: Box<dyn Fn() -> Entity>,
}

impl ClickDrawTool {
    pub fn new(build_drawable: impl Fn() -> Entity + 'static) -> ClickDrawTool {
        Self {
            hook: Default::default(),
            build_drawable: Box::new(build_drawable),
            drawable: None,
        }
    }

    pub fn end_drawing(&mut self) {
        let Some(mut drawable) = self.drawable.take() else {
            return;
        };

        let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
        (click_draw.finish)(&mut drawable);

        self.hook.end_drawing.dispatch(drawable);
    }
}

impl Tool for ClickDrawTool {
    fn interact(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => match &mut self.drawable {
                None => {
                    let mut drawable: Entity = (self.build_drawable)();

                    let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
                    (click_draw.mouse_down)(&mut drawable, &position);
                    self.hook.pointer_down.dispatch(position);

                    self.drawable = Some(drawable);
                }
                Some(drawable) => {
                    let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
                    (click_draw.mouse_down)(drawable, &position);
                }
            },
            Interaction::PointerMove(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
                (click_draw.mouse_move)(drawable, &position);
                self.hook.pointer_move.dispatch(position);
            }
            Interaction::PointerUp(position, _) => {
                self.hook.pointer_up.dispatch(position);
            }

            Interaction::KeyDown(Key::Esc) => {
                self.end_drawing()
            }
            Interaction::KeyDown(_) => {}
            Interaction::KeyUp(_) => {}
        }
    }
}
