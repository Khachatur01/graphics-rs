use geometry::shape::point::Point;
use geometry::shape::Shape;
use getter_methods::GetterMethods;

#[derive(GetterMethods, Default)]
struct Rotation {
    angle: f64,
    reference_point: Point,
}

#[derive(GetterMethods)]
pub struct ElementView<Id> {
    id: Id,
    shape: Shape,
    style: String,
    rotation: Rotation,
}

impl<Id> ElementView<Id> {
    pub fn from_shape(id: Id, element: Shape) -> Self {
        Self {
            id,
            shape: element,
            style: String::from(""),
            rotation: Default::default(),
        }
    }
}
