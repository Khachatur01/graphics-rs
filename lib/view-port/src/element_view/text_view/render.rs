use crate::element_view::text_view::TextView;
use rendering::{Render, Renderer};

impl<Id> Render for TextView<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
