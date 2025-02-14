use getter_methods::GetterMethods;
use crate::shape::point::Point;

#[derive(GetterMethods)]
pub struct Segment {
    start: Point,
    end: Point,
}
