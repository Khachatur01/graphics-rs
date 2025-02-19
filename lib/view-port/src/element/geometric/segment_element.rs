mod draw;
mod render;

use crate::element::ViewPortElement;
use geometry::figure::segment::Segment;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct SegmentElement<Id> {
    id: Id,
    segment: Segment,
    style: String,
}

impl<Id> SegmentElement<Id> {
    pub fn new(id: Id, segment: Segment) -> Self {
        Self {
            id,
            segment,
            style: String::from(""),
        }
    }
}

impl<Id> ViewPortElement<Id> for SegmentElement<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
