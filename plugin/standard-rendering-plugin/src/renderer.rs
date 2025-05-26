use crate::style::shape_style::ShapeStyle;
use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;

pub trait Renderable {
    fn render(&self, renderer: &mut dyn Renderer);
}

pub trait Renderer {
    fn clear(&mut self);
    fn segment(&mut self, segment: &Segment, style: &ShapeStyle);
    fn polygon(&mut self, polygon: &Polygon, style: &ShapeStyle);
    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle);
    fn circle(&mut self, circle: &Circle, style: &ShapeStyle);
    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle);
}
