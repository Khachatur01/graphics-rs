use crate::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn intersects_segment(&self, other: &Self) -> bool {
        todo!();
    }
}
