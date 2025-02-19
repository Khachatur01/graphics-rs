use crate::element::geometric::rectangle_element::RectangleElement;
use rendering::{Render, Renderer};

impl<Id> Render for RectangleElement<Id> {
    fn render(&mut self, renderer: &mut impl Renderer) {
        todo!()
    }
}
