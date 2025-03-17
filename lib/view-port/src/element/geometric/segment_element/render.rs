use crate::element::geometric::segment_element::SegmentElement;
use rendering::{Render, Renderer};

impl<Id> Render for SegmentElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
