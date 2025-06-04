use getter_methods::GetterMethods;
use crate::figure::point::Point;

#[derive(GetterMethods)]
pub struct Triangle {
    p0: Point,
    p1: Point,
    p2: Point,
}
