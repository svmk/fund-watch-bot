use crate::telegram::model::command::Command;
use crate::prelude::*;
use std::str::FromStr;

pub struct IncomingMessage {
    command: Command,
    argument: Option<String>,
}

impl IncomingMessage {
    pub fn get_command(&self) -> &Command {
        return &self.command;
    }

    pub fn get_argument(&self) -> Option<&String> {
        return self.argument.as_ref();
    }
}

impl FromStr for IncomingMessage {
    type Err = Failure;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut argument = None;
        let command = match text.starts_with("/") {
            true => {
                let text = &text[1..];
                let whitespace_index = text.find(|c: char| {
                    return c.is_whitespace();
                }).unwrap_or(0);
                let (command_text, arument_text) = text.split_at(whitespace_index);
                if !arument_text.is_empty() {
                    argument = Some(arument_text.to_string());
                }
                Command::from_str(command_text)?
            },
            false => {
                Command::Unknown
            },
        };
        let message = IncomingMessage {
            command,
            argument,
        };
        return Ok(message);
    }
}