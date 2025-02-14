use geometry::traits::{Drag, Resize};

trait Shape: Resize + Drag {}

pub struct ElementView<Id> {
    id: Id,
    shape: Box<dyn Shape>,
}
