mod draw;
mod render;

use crate::element_view::ElementView;
use geometry::figure::mesh::Mesh;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct MeshElement<Id> {
    id: Id,
    mesh: Mesh,
}

impl<Id> MeshElement<Id> {
    pub fn new(id: Id, mesh: Mesh) -> Self {
        Self {
            id,
            mesh,
        }
    }
}

impl<Id> ElementView<Id> for MeshElement<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
