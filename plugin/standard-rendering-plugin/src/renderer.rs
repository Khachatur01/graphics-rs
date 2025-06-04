use crate::style::shape_style::ShapeStyle;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
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
    // fn triangles_3d(&mut self, triangle: &[(Triangle, SurfaceStyle)]);
    // fn triangles_3d(&mut self, triangle: &[(Triangle, SurfaceStyle)], camera: &Camera, light: &Light);
}

pub trait IncrementalRenderer<Id: EntityId> {
    fn remove(&mut self, id: &str);
    fn add(&mut self, entity: &Entity<Id>);
    fn modify(&mut self, entity: &Entity<Id>);
}
