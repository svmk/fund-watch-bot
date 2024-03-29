use crate::telegram::model::command::Command;
use crate::telegram::service_handlers::command_handler::CommandHandler;
use crate::prelude::*;
use std::collections::BTreeMap;

pub struct CommandRouter {
    commands: BTreeMap<Command, Box<dyn CommandHandler>>,
}

impl CommandRouter {
    pub fn new() -> CommandRouter {
        return CommandRouter {
            commands: BTreeMap::new(),
        };
    }

    pub fn register_command(&mut self, command: Command, handler: impl CommandHandler + 'static) {
        let handler = Box::new(handler);
        let _ = self.commands.insert(command, handler);
    }

    pub fn get_command(&self, command: &Command) -> Result<&dyn CommandHandler, Failure> {
        if let Some(command_handler) = self.commands.get(command) {
            return Ok(command_handler.as_ref());
        }
        return crate::fail!("Unknown command `{:?}`", command);
    }
}