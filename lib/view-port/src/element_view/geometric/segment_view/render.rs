use crate::element_view::geometric::segment_view::SegmentView;
use rendering::{Render, Renderer};

impl<Id> Render for SegmentView<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
