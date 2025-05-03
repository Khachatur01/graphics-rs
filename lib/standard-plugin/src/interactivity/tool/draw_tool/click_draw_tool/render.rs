use crate::interactivity::tool::draw_tool::click_draw_tool::ClickDrawTool;
use rendering::{Renderable, Renderer};

impl<Id> Renderable for ClickDrawTool<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
