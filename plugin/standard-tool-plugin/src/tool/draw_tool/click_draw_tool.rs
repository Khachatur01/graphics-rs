mod render;

use core::entity::Entity;
use core::interactivity::Interactive;
use geometry::figure::point::Point;
use crate::ClickDraw;
use crate::traits::AddEntity;
use crate::tool::Tool;

pub struct ClickDrawTool {
    drawable: Option<Entity>,
    build_drawable: Box<dyn Fn() -> Entity>,
    view_port: Box<dyn AddEntity>,
}

impl ClickDrawTool {
    pub fn new(view_port: impl AddEntity + 'static, build_drawable: impl Fn() -> Entity + 'static) -> ClickDrawTool {
        Self {
            build_drawable: Box::new(build_drawable),
            drawable: None,
            view_port: Box::new(view_port),
        }
    }

    pub fn end_drawing(&mut self) {
        let Some(drawable) = self.drawable.take() else {
            return;
        };

        self.view_port.add_entity(drawable);
    }
}

impl Interactive for ClickDrawTool {
    fn mouse_down(&mut self, point: &Point) {
        match &mut self.drawable {
            None => {
                let mut drawable: Entity = (self.build_drawable)();

                let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
                (click_draw.mouse_down)(&mut drawable, point);

                self.drawable = Some(drawable);
            }
            Some(drawable) => {
                let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
                (click_draw.mouse_down)(drawable, point);
            }
        }
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(drawable) = &mut self.drawable else {
            return;
        };

        let click_draw: &ClickDraw = drawable.query().expect("Failed to query ClickDraw");
        (click_draw.mouse_move)(drawable, point);
    }

    fn mouse_up(&mut self, point: &Point) {

    }
}

impl Tool for ClickDrawTool {}