use crate::tool::draw_tool::click_draw_tool::ClickDrawTool;
use crate::tool::draw_tool::draw_mode::ClickDraw;
use rendering::{Render, Renderer};

impl<Drawable: ClickDraw> Render for ClickDrawTool<Drawable> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
