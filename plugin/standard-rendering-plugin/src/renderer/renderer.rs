pub mod camera;
pub mod light;

use crate::renderer::renderer::camera::Camera;
use crate::renderer::renderer::light::Light;
use crate::style::shape_style::ShapeStyle;
use algebra::linear::matrix::Matrix;
use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::path::Path;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use geometry::figure::triangle::Triangle;
use geometry::point::point_2d::Point2D;
use geometry::point::point_3d::Point3D;

pub trait Renderer {
    fn clear(&mut self);

    fn path(&mut self, path: &Path, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn segment_2d(&mut self, segment: &Segment<Point2D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn polygon_2d(&mut self, polygon: &Polygon<Point2D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn triangle_2d(&mut self, polygon: &Triangle<Point2D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn circle(&mut self, circle: &Circle, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);

    fn segment_3d(&mut self, segment: &Segment<Point3D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn polygon_3d(&mut self, polygon: &Polygon<Point3D>, style: &ShapeStyle, transform_matrix: Option<Matrix<3>>);
    fn triangles_3d(&mut self, triangles: &[(&Triangle<Point3D>, &ShapeStyle)], camera: &Camera, light: &Light, transform_matrix: Option<Matrix<3>>);
}
