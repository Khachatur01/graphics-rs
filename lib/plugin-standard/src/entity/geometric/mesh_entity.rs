mod draw;
mod render;

use geometry::figure::mesh::Mesh;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct MeshEntity {
    mesh: Mesh,
}

impl MeshEntity {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
        }
    }
}
