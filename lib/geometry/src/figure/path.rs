use crate::figure::path::command::Command;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};

pub mod command;

#[derive(Serialize, Deserialize, Clone, GetterMethods)]
pub struct Path {
    commands: Vec<Command>,
}

impl Path {
    pub fn new(commands: &[Command]) -> Self {
        Self {
            commands: commands.to_vec(),
        }
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

    pub fn to_svg_path(&self) -> String {
        self.commands.iter()
            .map(|command| {
                match command {
                    Command::MoveTo(move_to) => format!("{} {},{}", "M", move_to.to_point.x, move_to.to_point.y),
                    Command::LineTo(line_to) => format!("{} {},{}", "L", line_to.to_point.x, line_to.to_point.y),
                    Command::HorizontalLineTo(horizontal_line_to) => format!(""),
                    Command::VerticalLineTo(vertical_line_to) => format!(""),
                    Command::BezierTo(bezier_to) => format!(""),
                    Command::ArcTo(arc_to) => format!(""),
                    Command::Close => format!(""),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
