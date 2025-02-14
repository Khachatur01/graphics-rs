use crate::model_2d::path::command::Command;

pub mod command;

pub struct Path {
    commands: Vec<Command>,
}
