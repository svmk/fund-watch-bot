use crate::prelude::*;
use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_ref::ActionRef;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRoute {
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="action_ref")]
    action_ref: ActionRef,
}

impl ActionRoute {
    pub fn new(action_id: ActionId) -> ActionRoute {
        return ActionRoute {
            action_id,
            action_ref: ActionRef::new(),
        }
    }

    pub fn get_action_id(&self) -> &ActionId {
        return &self.action_id;
    }
}

impl FromStr for ActionRoute {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dot_index = match s.find('.') {
            Some(dot_index) => dot_index,
            None => {
                return crate::fail!("Unable to find dot at action route");
            },
        };
        let (action_id, action_ref) = s.split_at(dot_index);
        let action_id = ActionId::from_str(action_id)?;
        let action_ref = ActionRef::from_str(action_ref)?;
        let action_route = ActionRoute {
            action_id,
            action_ref,
        };
        return Ok(action_route);
    }
}

impl fmt::Display for ActionRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.action_id, self.action_ref)
    }
}