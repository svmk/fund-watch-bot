use crate::telegram::model::action_type::ActionType;
use crate::telegram::service_handlers::action_handler::ActionHandler;
use crate::prelude::*;
use std::collections::BTreeMap;

pub struct ActionRouter {
    actions: BTreeMap<ActionType, Box<dyn ActionHandler>>,
}

impl ActionRouter {
    pub fn new() -> ActionRouter {
        return ActionRouter {
            actions: BTreeMap::new(),
        };
    }

    pub fn register_action(&mut self, action: ActionType, handler: impl ActionHandler + 'static) {
        let handler = Box::new(handler);
        let _ = self.actions.insert(action, handler);
    }

    pub fn get_action_handler(&self, action: &ActionType) -> Result<&dyn ActionHandler, Failure> {
        if let Some(action_handler) = self.actions.get(action) {
            return Ok(action_handler.as_ref());
        }
        return crate::fail!("Unknown action `{:?}`", action);
    }
}