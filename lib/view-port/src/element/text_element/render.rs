use crate::element::text_element::TextElement;
use rendering::{Render, Renderer};

impl<Id> Render for TextElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
