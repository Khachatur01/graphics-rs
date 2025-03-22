mod draw;
mod render;

use crate::element_view::ElementView;
use geometry::figure::segment::Segment;
use getter_methods::GetterMethods;
use rendering::{Render, Renderer};

#[derive(GetterMethods)]
pub struct SegmentView<Id> {
    id: Id,
    segment: Segment,
    style: String,
}

impl<Id> SegmentView<Id> {
    pub fn new(id: Id, segment: Segment) -> Self {
        Self {
            id,
            segment,
            style: String::from(""),
        }
    }
}

impl<Id> ElementView<Id> for SegmentView<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
