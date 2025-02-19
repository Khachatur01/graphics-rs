use crate::element::geometric::mesh_element::MeshElement;
use rendering::{Render, Renderer};

impl<Id> Render for MeshElement<Id> {
    fn render(&mut self, renderer: &mut impl Renderer) {
        todo!()
    }
}
