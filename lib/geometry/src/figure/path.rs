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

    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn insert_command(&mut self, index: usize, command: Command) -> Result<(), ()> {
        if index > self.commands.len() {
            return Err(());
        }

        self.commands.insert(index, command);

        Ok(())
    }

    pub fn remove_command(&mut self, index: usize) -> Result<(), ()> {
        if index > self.commands.len() {
            return Err(());
        }

        self.commands.remove(index);

        Ok(())
    }
}
