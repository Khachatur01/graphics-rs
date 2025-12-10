use crate::figure::rectangle::Rectangle;
use crate::figure::segment::Segment;
use crate::point::point_2d::Point2D;
use crate::point::Point;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, GetterMethods)]
pub struct Polygon<P> {
    vertices: Vec<P>,
}

impl<P: Point> Polygon<P> {
    pub fn new(vertices: &[P]) -> Polygon<P> {
        Polygon {
            vertices: vertices.to_vec(),
        }
    }

    pub fn add_vertex(&mut self, vertex: P) {
        self.vertices.push(vertex);
    }

    pub fn replace_vertex(&mut self, index: usize, vertex: P) {
        self.vertices[index] = vertex;
    }

    pub fn replace_last_vertex(&mut self, vertex: P) {
        let length: usize = self.vertices.len();

        self.vertices[length - 1] = vertex;
    }

    pub fn remove_last_vertex(&mut self) {
        self.vertices.pop();
    }
}

impl Polygon<Point2D> {
    pub fn is_inside_rectangle(&self, rectangle: &Rectangle) -> bool {
        let point_outside_rect = self.vertices().iter().find(|vertex|
            vertex.x() < rectangle.top_left.x ||
            vertex.x() > rectangle.top_left.x + rectangle.width ||
            vertex.y() < rectangle.top_left.y ||
            vertex.y() > rectangle.top_left.y + rectangle.height
        );

        /* Polygon is inside rectangle if there is not any point which is outside the rectangle */
        point_outside_rect.is_none()
    }

    pub fn intersects_rectangle(&self, rectangle: &Rectangle) -> bool {
        let polygon_segments = self
            .vertices
            .windows(2)
            .map(|points| (points[0], points[1]))
            .map(|(start, end)| Segment::new(start.clone(), end.clone()))
            .collect::<Vec<_>>();

        let rectangle_segments: [Segment<Point2D>; 4] = rectangle.into();

        for rectangle_segment in &rectangle_segments {
            for polygon_segment in polygon_segments.iter() {
                if rectangle_segment.intersects_segment(polygon_segment) {
                    return true;
                }
            }
        }

        false
    }
}
