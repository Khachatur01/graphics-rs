use crate::interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use rendering_plugin::feature::Render;
use rendering_plugin::{Renderable, Renderer};

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
