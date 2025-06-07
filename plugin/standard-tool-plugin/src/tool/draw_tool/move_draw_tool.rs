use crate::tool::{Interaction, Tool};
use crate::MoveDraw;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use event_handler::{make_event_handler, EventChannel, Receiver};
use geometry::point::point_2d::Point2D;

mod render;

make_event_handler!(
    EventHandler<Id: EntityId>,
    pointer_down: Point2D,
    pointer_move: Point2D,
    pointer_up: Point2D,
    end_drawing: Entity<Id>
);

pub struct MoveDrawTool<Id: EntityId> {
    pub event: EventHandler<Id>,

    start: Option<Point2D>,
    drawable: Option<Entity<Id>>,
    build_drawable: Box<dyn Fn() -> Entity<Id>>,
}

impl<Id: EntityId> MoveDrawTool<Id> {
    pub fn new(build_drawable: impl Fn() -> Entity<Id> + 'static) -> MoveDrawTool<Id> {
        Self {
            start: None,
            drawable: None,
            build_drawable: Box::new(build_drawable),

            event: Default::default(),
        }
    }

    fn end_drawing(&mut self) {
        self.start.take();

        let Some(drawable) = self.drawable.take() else {
            return;
        };

        self.event.end_drawing.dispatch(drawable);
    }
}

impl<Id: EntityId> Tool for MoveDrawTool<Id> {
    fn interact(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => {
                let mut drawable: Entity<Id> = (self.build_drawable)();
                self.start.replace(position.clone());

                let move_draw: &MoveDraw<Id> = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.pointer_down)(&mut drawable, &position);
                self.event.pointer_down.dispatch(position);

                self.drawable = Some(drawable);
            }
            Interaction::PointerMove(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                let Some(start) = self.start else {
                    return;
                };

                let move_draw: &MoveDraw<Id> = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.pointer_move)(drawable, &start, &position);
                self.event.pointer_move.dispatch(position);
            }
            Interaction::PointerUp(position, _) => {
                let Some(drawable) = &mut self.drawable else {
                    return;
                };

                /* Take value from start point to make sure after mouse up action start point is None */
                let Some(start) = self.start.take() else {
                    return;
                };

                let move_draw: &MoveDraw<Id> = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.pointer_up)(drawable, &start, &position);
                self.event.pointer_up.dispatch(position);

                self.end_drawing();
            }

            Interaction::KeyDown(_) => {}
            Interaction::KeyUp(_) => {}
        }
    }
}
