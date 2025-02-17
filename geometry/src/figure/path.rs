use crate::figure::path::command::Command;
use getter_methods::GetterMethods;

pub mod command;

#[derive(GetterMethods)]
pub struct Path {
    commands: Vec<Command>,
}

impl Path {
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}
