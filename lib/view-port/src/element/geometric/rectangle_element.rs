mod render;
mod draw;

use crate::element::ViewPortElement;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
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

impl<Id> ViewPortElement<Id> for RectangleElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
