use crate::app::model::encoded_uint::EncodedUint;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as SerdeError};
use crate::telegram::model::action_type::ActionType;
use crate::telegram::model::action_route::ActionRoute;
use crate::repository::model::identity::Identity;
use crate::prelude::*;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionId {
    action_type: ActionType,
    action_id: EncodedUint,
}

impl ActionId {
    pub fn new(action_type: ActionType) -> ActionId {
        return ActionId {
            action_type,
            action_id: EncodedUint::new(),
        }
    }

    pub fn create_route(&self) -> ActionRoute {
        return ActionRoute::new(self.clone());
    }

    pub fn get_action_type(&self) -> &ActionType {
        return &self.action_type;
    }
}

impl fmt::Display for ActionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.action_type, self.action_id)
    }
}

impl FromStr for ActionId {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dot_index = match s.find('.') {
            Some(dot_index) => dot_index,
            None => {
                return crate::fail!("Unable to find dot at action id");
            },
        };
        let (action_type, action_id) = s.split_at(dot_index);
        let action_id = &action_id[1..];
        let action_type = ActionType::from_str(action_type)?;
        let action_id = EncodedUint::from_str(action_id)?;
        let action_route = ActionId {
            action_type,
            action_id,
        };
        return Ok(action_route);
    }
}

impl Serialize for ActionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
            let value = format!("{}", self);
            return value.serialize(serializer);
        }
}

impl <'de>Deserialize<'de> for ActionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?;
            let value = ActionId::from_str(&value).map_err(SerdeError::custom)?;
            return Ok(value);
        }
}

impl Identity for ActionId {
    fn to_string(&self) -> String {
        return format!("{}", self);
    }
}