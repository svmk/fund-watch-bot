use crate::repository::model::identity::Identity;
use serde::{Serialize, de::DeserializeOwned};

pub trait Entity: Serialize + DeserializeOwned {
    type Id: Identity;
    fn get_entity_id(&self) -> &Self::Id;
}
