mod render;
mod draw;

use crate::element_view::ElementView;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;

#[derive(GetterMethods, Clone)]
pub struct RectangleElement<Id> {
    id: Id,
    rectangle: Rectangle,
    style: String,
}

impl<Id> RectangleElement<Id> {
    pub fn new(id: Id, rectangle: Rectangle) -> Self {
        Self {
            id,
            rectangle,
            style: String::from(""),
        }
    }
}

impl<Id> ElementView<Id> for RectangleElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl<Id> RectangleElement<Id> {
    pub fn zero_sized(id: Id) -> Self {
        Self {
            id,
            rectangle: Rectangle::zero_sized(Point::default()),
            style: String::from(""),
        }
    }
}
