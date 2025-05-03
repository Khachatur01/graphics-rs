use crate::interactivity::tool::draw_tool::MoveDraw;
use crate::interactivity::tool::Tool;
use core::entity::Entity;
use core::interactivity::Interactive;
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
        self.drawable.take();
        self.start.take();
    }
}

impl Interactive for MoveDrawTool {
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

impl Tool for MoveDrawTool {}
