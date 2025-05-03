use crate::interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use plugin_rendering::behaviour::Render;
use plugin_rendering::{Renderable, Renderer};

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
