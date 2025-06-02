use entity_model_feature::entity::Entity;
use crate::tool::{Interaction, Tool};
use crate::MoveDraw;
use event_handler::EventHandler;
use geometry::figure::point::Point;

mod render;

#[derive(Default)]
pub struct Hook {
    pub pointer_down: EventHandler<Point>,
    pub pointer_move: EventHandler<Point>,
    pub pointer_up: EventHandler<Point>,
    pub end_drawing: EventHandler<Entity>
}

pub struct MoveDrawTool {
    pub hook: Hook,

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

            hook: Default::default(),
        }
    }

    fn end_drawing(&mut self) {
        self.start.take();

        let Some(drawable) = self.drawable.take() else {
            return;
        };

        self.hook.end_drawing.dispatch(drawable);
    }
}

impl Tool for MoveDrawTool {
    fn interact(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::PointerDown(position, _) => {
                let mut drawable: Entity = (self.build_drawable)();
                self.start.replace(position.clone());

                let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
                (move_draw.mouse_down)(&mut drawable, &position);
                self.hook.pointer_down.dispatch(position);

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
                self.hook.pointer_move.dispatch(position);
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
                self.hook.pointer_up.dispatch(position);

                self.end_drawing();
            }

            Interaction::KeyDown(_) => {}
            Interaction::KeyUp(_) => {}
        }
    }
}
