use crate::figure::path::command::positioning::Positioning;
use crate::figure::point::Point;
use crate::math::Drag;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct MoveTo {
    to_point: Point,
    positioning: Positioning,
}

impl Drag for MoveTo {
    fn drag(&mut self, delta: &Point) {
        self.to_point += delta;
    }
}
