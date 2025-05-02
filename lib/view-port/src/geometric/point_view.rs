mod render;
mod draw;

use element_view::ElementView;
use geometry::figure::point::Point;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;
use std::any::Any;

#[derive(GetterMethods)]
pub struct PointElement<Id> {
    id: Id,
    point: Point,
    style: ShapeStyle,
    behaviors: Vec<Box<dyn Any>>,
}

impl<Id> PointElement<Id> {
    pub fn new(id: Id, point: Point, style: ShapeStyle) -> Self {
        Self {
            id,
            point,
            style,
            behaviors: vec![],
        }
    }
}

impl<Id> ElementView<Id> for PointElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
