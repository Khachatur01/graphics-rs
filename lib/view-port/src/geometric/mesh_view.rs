mod draw;
mod render;

use element_view::ElementView;
use geometry::figure::mesh::Mesh;
use getter_methods::GetterMethods;
use std::any::Any;

#[derive(GetterMethods)]
pub struct MeshElement<Id> {
    id: Id,
    mesh: Mesh,
    behaviors: Vec<Box<dyn Any>>,
}

impl<Id> MeshElement<Id> {
    pub fn new(id: Id, mesh: Mesh) -> Self {
        Self {
            id,
            mesh,
            behaviors: vec![],
        }
    }
}

impl<Id> ElementView<Id> for MeshElement<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
