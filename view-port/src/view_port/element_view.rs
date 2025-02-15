use geometry::shape::point::Point;
use geometry::shape::Shape;

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
