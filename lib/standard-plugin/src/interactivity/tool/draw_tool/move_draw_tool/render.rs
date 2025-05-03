use crate::interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use rendering::{Renderable, Renderer};

impl<Id: 'static> Renderable for MoveDrawTool<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
