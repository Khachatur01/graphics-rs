use crate::element::geometric::rectangle_element::RectangleElement;
use rendering::{Render, Renderer};

impl<Id> Render for RectangleElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
