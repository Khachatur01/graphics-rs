use geometry::shape::point::Point;
use geometry::shape::Shape;

#[derive(Default)]
struct Rotation {
    angle: f64,
    reference_point: Point,
}

pub struct ElementView<Id> {
    id: Id,
    element: Box<dyn Shape>,
    style: String,
    rotation: Rotation,
}

impl<Id> ElementView<Id> {
    pub fn new(id: Id, element: Box<dyn Shape>) -> Self {
        Self {
            id,
            element,
            style: String::from(""),
            rotation: Default::default(),
        }
    }
}
