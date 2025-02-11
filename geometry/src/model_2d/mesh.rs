use crate::model_2d::point::Point;

pub struct Edge {
    p0_index: u8,
    p1_index: u8,
}

pub struct Mesh {
    vertices: Vec<Point>,
    edges: Vec<Edge>
}
