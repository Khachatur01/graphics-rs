use getter_methods::GetterMethods;
use crate::shape::path::command::Command;

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
