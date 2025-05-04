use crate::interactivity::tool::draw_tool::click_draw_tool::ClickDrawTool;
use rendering_plugin::feature::Render;
use rendering_plugin::{Renderable, Renderer};

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
