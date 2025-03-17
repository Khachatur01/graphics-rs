use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::draw_tool::move_draw_tool::MoveDrawTool;
use rendering::{Render, Renderer};

impl<Drawable: MoveDraw> Render for MoveDrawTool<Drawable> {
    fn render(&self, renderer: &mut dyn Renderer) {
        self.drawable.render(renderer);
    }
}