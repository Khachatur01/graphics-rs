use crate::model_2d::path::command::Command as PathCommand;
use crate::model_2d::point::Point;

pub struct Edge {
    start_point_index: u8,
    end_point_index: u8,
    path_command: PathCommand
}

pub struct Mesh {
    knots: Vec<Point>,
    edges: Vec<Edge>
}
