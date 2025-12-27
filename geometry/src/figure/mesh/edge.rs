use crate::figure::path::command::Command as PathCommand;

#[derive(Clone)]
pub struct Edge {
    start_point_index: u8,
    end_point_index: u8,
    path_command: PathCommand,
}

impl Edge {
    pub fn new(start_point_index: u8, end_point_index: u8, path_command: PathCommand) -> Self {
        Edge {
            start_point_index,
            end_point_index,
            path_command,
        }
    }
}
