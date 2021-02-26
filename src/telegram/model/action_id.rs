use crate::app::model::encoded_uint::EncodedUint;
use crate::repository::model::identity::Identity;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "ActionId::from_encoded_uint", from_str_derive = true)]
pub struct ActionId(EncodedUint);

impl ActionId {
    fn from_encoded_uint(value: EncodedUint) -> Result<ActionId, Failure> {
        return Ok(ActionId(value));
    }
}

impl Identity for ActionId {
    fn to_string(&self) -> String {
        return format!("{}", self);
    }
}