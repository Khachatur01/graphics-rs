mod render;
mod draw;

use geometry::figure::point::Point;
use getter_methods::GetterMethods;
use rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods)]
pub struct PointEntity {
    point: Point,
    style: ShapeStyle,
}

impl PointEntity {
    pub fn new(point: Point, style: ShapeStyle) -> Self {
        Self {
            point,
            style,
        }
    }
}
