use crate::interactivity::tool::draw_tool::click_draw_tool::ClickDrawTool;
use plugin_rendering::behaviour::Render;
use plugin_rendering::{Renderable, Renderer};

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
