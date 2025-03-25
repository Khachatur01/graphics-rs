mod render;
mod draw;

use crate::element_view::{DefaultWithId, ElementView};
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods, Clone)]
pub struct RectangleElement<Id> {
    id: Id,
    rectangle: Rectangle,
    style: ShapeStyle,
}

impl<Id> RectangleElement<Id> {
    pub fn new(id: Id, rectangle: Rectangle) -> Self {
        Self {
            id,
            rectangle,
            style: ShapeStyle::default(),
        }
    }
}

impl<Id> ElementView<Id> for RectangleElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl<Id> DefaultWithId<Id> for RectangleElement<Id> {
    fn default_with_id(id: Id) -> Self {
        Self::new(
            id,
            Rectangle::zero_sized(Point::default())
        )
    }
}
