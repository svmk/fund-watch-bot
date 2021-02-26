use crate::app::model::encoded_uint::EncodedUint;
use crate::repository::model::identity::Identity;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "ActionRef::from_encoded_uint", from_str_derive = true)]
pub struct ActionRef(EncodedUint);

impl ActionRef {
    fn from_encoded_uint(value: EncodedUint) -> Result<ActionRef, Failure> {
        return Ok(ActionRef(value));
    }
}

impl Identity for ActionRef {
    fn to_string(&self) -> String {
        return format!("{}", self);
    }
}