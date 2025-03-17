use crate::element::geometric::point_element::PointElement;
use rendering::{Render, Renderer};

impl<Id> Render for PointElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
