use crate::telegram::model::command::Command;
use crate::prelude::*;
use std::str::FromStr;

pub struct IncomingMessage {
    command: Command,
}

impl FromStr for IncomingMessage {
    type Err = Failure;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let command = match text.starts_with("/") {
            true => {
                let text = &text[1..];
                let whitespace_index = text.find(|c: char| {
                    return c.is_whitespace();
                }).unwrap_or(0);
                let (command_text, _) = text.split_at(whitespace_index);
                Command::from_str(command_text)?
            },
            false => {
                Command::Unknown
            },
        };
        let message = IncomingMessage {
            command,
        };
        return Ok(message);
    }
}