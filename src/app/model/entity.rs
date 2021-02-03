use crate::app::model::identity::Identity;
use serde::{Serialize, de::DeserializeOwned};

pub trait Entity<Id: Identity>: Serialize + DeserializeOwned {
    fn get_entity_id(&self) -> &Id;
}