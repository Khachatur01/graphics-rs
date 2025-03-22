use crate::element_view::geometric::rectangle_view::RectangleElement;
use crate::identifier::Identifier;
use rendering::{Render, Renderer};

impl<Id: Identifier> Render for RectangleElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.rectangle(self.id().to_string().as_str(), self.rectangle());
    }
}
