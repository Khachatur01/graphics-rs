mod render;
mod draw;

use crate::element_view::ElementView;
use geometry::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct PointElement<Id> {
    id: Id,
    point: Point,
    style: String,
}

impl<Id> PointElement<Id> {
    pub fn new(id: Id, point: Point) -> Self {
        Self {
            id,
            point,
            style: String::from(""),
        }
    }
}

impl<Id> ElementView<Id> for PointElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
