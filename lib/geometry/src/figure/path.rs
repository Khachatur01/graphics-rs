use crate::figure::path::command::Command;
use crate::figure::point::Point;
use crate::math::Drag;
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

    pub fn drag_command(&mut self, index: usize, delta: &Point) -> Result<(), ()> {
        if index > self.commands.len() {
            return Err(());
        }

        self.commands[index].drag(delta);

        Ok(())
    }
}

impl Drag for Path {
    fn drag(&mut self, delta: &Point) {
        for command in self.commands.iter_mut() {
            command.drag(delta);
        }
    }
}
