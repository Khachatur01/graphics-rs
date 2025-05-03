mod draw;
mod render;

use geometry::figure::segment::Segment;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods)]
pub struct SegmentEntity {
    segment: Segment,
    style: ShapeStyle,
}

impl SegmentEntity {
    pub fn new(segment: Segment, style: ShapeStyle) -> Self {
        Self {
            segment,
            style,
        }
    }
}
