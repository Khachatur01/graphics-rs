use crate::interactivity::tool::select_tool::SelectTool;
use rendering::{Renderable, Renderer};

impl<Id> Renderable for SelectTool<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        
    }
}
