pub mod style;

use crate::style::shape_style::ShapeStyle;
use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use std::any::Any;

pub struct Render {
    pub render: fn(element: &dyn Any, renderer: &mut dyn Renderer),
}

pub trait Renderer {
    fn clear(&mut self);
    fn segment(&mut self, segment: &Segment, style: &ShapeStyle);
    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle);
    fn circle(&mut self, circle: &Circle, style: &ShapeStyle);
    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle);
}
