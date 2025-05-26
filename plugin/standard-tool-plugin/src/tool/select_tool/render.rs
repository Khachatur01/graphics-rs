use crate::tool::select_tool::SelectTool;
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use standard_rendering_plugin::style::color::Color;
use standard_rendering_plugin::style::shape_style::ShapeStyle;

impl Renderable for SelectTool {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(selection) = self.selection else {
            return;
        };

        renderer.rectangle(
            &selection,
            &ShapeStyle {
                stroke_color: Color(0, 0, 255, 120),
                stroke_dash_array: vec![2, 2],
                ..Default::default()
            },
        )
    }
}
