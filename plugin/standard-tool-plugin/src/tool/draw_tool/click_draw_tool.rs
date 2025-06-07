mod render;

use crate::tool::{Interaction, Key, Tool};
use crate::ClickDraw;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use event_handler::make_event_handler;
use geometry::point::point_2d::Point2D;

make_event_handler!(
    EventHandler<Id: EntityId>,
    pointer_down: Point2D,
    pointer_move: Point2D,
    pointer_up: Point2D,
    end_drawing: Entity<Id>
);

pub struct ClickDrawTool<Id: EntityId> {
    pub event: EventHandler<Id>,

    drawable: Option<Entity<Id>>,
    build_drawable: Box<dyn Fn() -> Entity<Id>>,
}

impl<Id: EntityId> ClickDrawTool<Id> {
    pub fn new(build_drawable: impl Fn() -> Entity<Id> + 'static) -> ClickDrawTool<Id> {
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

        let click_draw: &ClickDraw<Id> = drawable.query().expect("Failed to query ClickDraw");
        (click_draw.finish)(&mut drawable);

        self.event.end_drawing.dispatch(drawable);
    }
}

impl<Id: EntityId> Tool for ClickDrawTool<Id> {
    fn interact(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => match &mut self.drawable {
                None => {
                    let mut drawable: Entity<Id> = (self.build_drawable)();

                    let click_draw: &ClickDraw<Id> = drawable.query().expect("Failed to query ClickDraw");
                    (click_draw.pointer_down)(&mut drawable, &position);
                    self.event.pointer_down.dispatch(position);

                    self.drawable = Some(drawable);
                }
                Some(drawable) => {
                    let click_draw: &ClickDraw<Id> = drawable.query().expect("Failed to query ClickDraw");
                    (click_draw.pointer_down)(drawable, &position);
                }
            },
            Interaction::PointerMove(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                let click_draw: &ClickDraw<Id> = drawable.query().expect("Failed to query ClickDraw");
                (click_draw.pointer_move)(drawable, &position);
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
