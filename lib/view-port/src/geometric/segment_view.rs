mod draw;
mod render;

use element_view::ElementView;
use geometry::figure::segment::Segment;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;
use std::any::Any;

#[derive(GetterMethods)]
pub struct SegmentView<Id> {
    id: Id,
    segment: Segment,
    style: ShapeStyle,
    behaviors: Vec<Box<dyn Any>>,
}

impl<Id> SegmentView<Id> {
    pub fn new(id: Id, segment: Segment, style: ShapeStyle) -> Self {
        Self {
            id,
            segment,
            style,
            behaviors: vec![],
        }
    }
}

impl<Id> ElementView<Id> for SegmentView<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
