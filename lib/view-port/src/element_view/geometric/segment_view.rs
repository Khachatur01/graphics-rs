mod draw;
mod render;

use crate::element_view::ElementView;
use geometry::figure::segment::Segment;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods)]
pub struct SegmentView<Id> {
    id: Id,
    segment: Segment,
    style: ShapeStyle,
}

impl<Id> SegmentView<Id> {
    pub fn new(id: Id, segment: Segment, style: ShapeStyle) -> Self {
        Self {
            id,
            segment,
            style,
        }
    }
}

impl<Id> ElementView<Id> for SegmentView<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
