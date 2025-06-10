use crate::tool::select_tool::SelectTool;
use entity_model_feature::entity_id::EntityId;
use standard_rendering_plugin::renderable::Renderable;
use standard_rendering_plugin::renderer::renderer::Renderer;
use standard_rendering_plugin::style::color::Color;
use standard_rendering_plugin::style::shape_style::{ShapeStyle, StrokeStyle};

impl<Id: EntityId> Renderable for SelectTool<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(selection) = self.selection else {
            return;
        };

        let stroke_color: Color =
            if selection.width() < 0.0 {
                Color(0, 128, 0, 255)
            } else {
                Color(21, 69, 255, 255)
            };

        renderer.rectangle(
            &selection,
            &ShapeStyle {
                stroke: StrokeStyle {
                    color: stroke_color,
                    width: 1.0,
                    dash_array: vec![2, 2],
                    antialiasing: false,
                },
                ..Default::default()
            },
        )
    }
}
