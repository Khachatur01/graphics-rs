use crate::tool::draw_tool::click_draw_tool::ClickDrawTool;
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use standard_rendering_plugin::Render;

impl Renderable for ClickDrawTool {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(drawable) = &self.drawable else {
            return;
        };

        if let Some(render) = drawable.query::<Render>() {
            (render.render)(drawable, renderer);
        }
    }
}
