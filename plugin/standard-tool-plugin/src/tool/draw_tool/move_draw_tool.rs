use core::entity::Entity;
use geometry::figure::point::Point;
use crate::MoveDraw;
use crate::traits::AddEntity;
use crate::tool::Tool;

mod render;

pub struct MoveDrawTool {
    start: Option<Point>,
    drawable: Option<Entity>,
    build_drawable: Box<dyn Fn() -> Entity>,
    view_port: Box<dyn AddEntity>,
}

impl MoveDrawTool {
    pub fn new(view_port: impl AddEntity + 'static, build_drawable: impl Fn() -> Entity + 'static) -> MoveDrawTool {
        Self {
            start: None,
            drawable: None,
            build_drawable: Box::new(build_drawable),
            view_port: Box::new(view_port),
        }
    }

    pub fn end_drawing(&mut self) {
        self.start.take();

        let Some(drawable) = self.drawable.take() else {
            return;
        };

        self.view_port.add_entity(drawable);
    }
}

impl Tool for MoveDrawTool {
    fn mouse_down(&mut self, point: &Point) {
        let mut drawable: Entity = (self.build_drawable)();
        self.start.replace(point.clone());

        let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
        (move_draw.mouse_down)(&mut drawable, point);

        self.drawable = Some(drawable);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(drawable) = &mut self.drawable else {
            return;
        };

        let Some(start) = self.start else { return; };

        let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
        (move_draw.mouse_move)(drawable, &start, point);
    }

    fn mouse_up(&mut self, point: &Point) {
        let Some(drawable) = &mut self.drawable else {
            return;
        };

        /* Take value from start point to make sure after mouse up action start point is None */
        let Some(start) = self.start.take() else { return; };

        let move_draw: &MoveDraw = drawable.query().expect("Failed to query MoveDraw");
        (move_draw.mouse_up)(drawable, &start, point);

        self.end_drawing();
    }
}
