use crate::interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use rendering::behaviour::Render;
use rendering::{Renderable, Renderer};

impl Renderable for MoveDrawTool {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(drawable) = &self.drawable else {
            return;
        };

        if let Some(render) = drawable.query::<Render>() {
            (render.render)(drawable, renderer);
        }
    }
}
