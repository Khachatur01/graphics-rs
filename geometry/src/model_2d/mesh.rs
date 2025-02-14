use getter_methods::GetterMethods;
use crate::model_2d::path::command::Command as PathCommand;
use crate::model_2d::point::Point;

#[derive(GetterMethods)]
pub struct Edge {
    start_point_index: u8,
    end_point_index: u8,
    path_command: PathCommand
}

impl Edge {
    pub fn new(start_point_index: u8, end_point_index: u8, path_command: PathCommand) -> Self {
        Edge { start_point_index, end_point_index, path_command }
    }
}

#[derive(GetterMethods)]
pub struct Mesh {
    knots: Vec<Point>,
    edges: Vec<Edge>
}

impl Mesh {
    pub fn new(knots: Vec<Point>, edges: Vec<Edge>) -> Mesh {
        Mesh { knots, edges }
    }
}
