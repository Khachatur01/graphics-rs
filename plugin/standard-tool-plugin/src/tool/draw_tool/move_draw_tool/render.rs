use standard_rendering_plugin::Render;
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use crate::tool::draw_tool::move_draw_tool::MoveDrawTool;

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
