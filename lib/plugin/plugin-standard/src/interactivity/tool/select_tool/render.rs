use crate::interactivity::tool::select_tool::SelectTool;
use rendering_plugin::style::color::Color;
use rendering_plugin::style::shape_style::ShapeStyle;
use rendering_plugin::{Renderable, Renderer};

impl Renderable for SelectTool {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(selection) = self.selection else {
            return;
        };

        renderer.rectangle(&selection, &ShapeStyle {
            stroke_color: Color(0, 0, 255, 120),
            stroke_dash_array: vec![2, 2],
            ..Default::default()
        })
    }
}
