mod render;

use entity_model_feature::entity::Entity;
use event_handler::{make_event_handler, EventChannel, Receiver};
use geometry::figure::point::Point;
use crate::tool::{Interaction, Key, Tool};
use crate::ClickDraw;

make_event_handler!(
    pointer_down: Point,
    pointer_move: Point,
    pointer_up: Point,
    end_drawing: Entity
);

pub struct ClickDrawTool {
    pub event: EventHandler,

    drawable: Option<Entity>,
    build_drawable: Box<dyn Fn() -> Entity>,
}

impl ClickDrawTool {
    pub fn new(build_drawable: impl Fn() -> Entity + 'static) -> ClickDrawTool {
        Self {
            event: Default::default(),
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

        self.event.end_drawing.dispatch(drawable);
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
                    self.event.pointer_down.dispatch(position);

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
                self.event.pointer_move.dispatch(position);
            }
            Interaction::PointerUp(position, _) => {
                self.event.pointer_up.dispatch(position);
            }

            Interaction::KeyDown(Key::Esc) => {
                self.end_drawing()
            }
            Interaction::KeyDown(_) => {}
            Interaction::KeyUp(_) => {}
        }
    }
}
