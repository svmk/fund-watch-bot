use std::collections::HashMap;

use crate::telegram::model::command::Command;

#[derive(Debug)]
pub struct BotCommandSettings {
    commands: HashMap<String, String>,
}

impl BotCommandSettings {
    pub fn new() -> BotCommandSettings {
        return BotCommandSettings {
            commands: HashMap::new(),
        }
    }

    pub fn register_description(&mut self, command: Command, text: impl Into<String>) {
        let command = format!("{}", command);
        let _ = self.commands.insert(command, text.into());
    }

    pub fn iter(&self) -> impl Iterator<Item=(&String, &String)> + '_ {
        return self.commands.iter();
    }
}