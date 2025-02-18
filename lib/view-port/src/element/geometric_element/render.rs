use crate::element::geometric_element::{GeometricElement, GeometricFigure};
use rendering::{Render, Renderer};

impl<Id, RendererImpl: Renderer> Render<RendererImpl> for GeometricElement<Id> {
    fn render(&mut self, renderer: &mut RendererImpl) {
        match &self.geometric_figure {
            GeometricFigure::Point(point) => todo!(),
            GeometricFigure::Segment(segment) => {
                renderer.segment(segment);
            }
        }
    }
}
