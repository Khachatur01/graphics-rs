use crate::element_view::geometric::rectangle_view::RectangleElement;
use rendering::{Render, Renderer};

impl<Id> Render for RectangleElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.rectangle(self.rectangle());
    }
}
