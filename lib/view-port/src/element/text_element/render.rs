use crate::element::text_element::TextElement;
use rendering::{Render, Renderer};

impl<Id, RendererImpl: Renderer> Render<RendererImpl> for TextElement<Id> {
    fn render(&mut self, renderer: &mut RendererImpl) {
        todo!()
    }
}
