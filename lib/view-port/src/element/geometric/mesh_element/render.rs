use crate::element::geometric::mesh_element::MeshElement;
use rendering::{Render, Renderer};

impl<Id> Render for MeshElement<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        todo!()
    }
}
