use crate::figure::point::Point;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Polygon {
    vertices: Vec<Point>
}

impl Polygon {
    pub fn new(vertices: &[Point]) -> Polygon {
        Polygon {
            vertices: vertices.to_vec()
        }
    }

    pub fn add_vertex(&mut self, vertex: Point) {
        self.vertices.push(vertex);
    }

    pub fn replace_vertex(&mut self, index: usize, vertex: Point) {
        self.vertices[index] = vertex;
    }

    pub fn replace_last_vertex(&mut self, vertex: Point) {
        let length: usize = self.vertices.len();

        self.vertices[length - 1] = vertex;
    }
}
