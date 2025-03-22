use crate::element_view::geometric::mesh_view::MeshElement;
use rendering::{Render, Renderer};

impl<Id> Render for MeshElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
