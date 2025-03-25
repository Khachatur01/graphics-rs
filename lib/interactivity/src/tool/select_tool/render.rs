use crate::tool::select_tool::SelectTool;
use rendering::style::shape_style::ShapeStyle;
use rendering::{Render, Renderer};

impl Render for SelectTool {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(selection) = self.selection else {
            return;
        };

        renderer.rectangle(&selection, &ShapeStyle::default());
    }
}
