use crate::shape::circle::Circle;
use crate::shape::point::Point;
use crate::shape::rectangle::Rectangle;
use crate::shape::segment::Segment;

pub mod point;
pub mod segment;
pub mod rectangle;
pub mod circle;
pub mod ellipse;
pub mod polygon;
pub mod polyline;
pub mod mesh;
pub mod path;

pub enum Shape {
    Point(Point),
    Segment(Segment),
    Rectangle(Rectangle),
    Circle(Circle),
}
