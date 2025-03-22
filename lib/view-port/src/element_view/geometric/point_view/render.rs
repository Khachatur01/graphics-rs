use crate::element_view::geometric::point_view::PointElement;
use rendering::{Render, Renderer};

impl<Id> Render for PointElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
