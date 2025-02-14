use getter_methods::GetterMethods;
use crate::model_2d::point::Point;

#[derive(GetterMethods)]
pub struct Segment {
    start: Point,
    end: Point,
}
