mod render;
mod draw;

use crate::element_view::ElementView;
use geometry::figure::point::Point;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods)]
pub struct PointElement<Id> {
    id: Id,
    point: Point,
    style: ShapeStyle,
}

impl<Id> PointElement<Id> {
    pub fn new(id: Id, point: Point, style: ShapeStyle) -> Self {
        Self {
            id,
            point,
            style,
        }
    }
}

impl<Id> ElementView<Id> for PointElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
