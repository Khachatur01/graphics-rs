use crate::tool::draw_tool::click_draw_tool::ClickDrawTool;
use entity_model_feature::entity_id::EntityId;
use standard_rendering_plugin::renderable::Renderable;
use standard_rendering_plugin::renderer::renderer::Renderer;
use standard_rendering_plugin::Render;

impl<Id: EntityId> Renderable for ClickDrawTool<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(drawable) = &self.drawable else {
            return;
        };

        if let Some(render) = drawable.query::<Render<Id>>() {
            (render.render)(drawable, renderer);
        }
    }
}
