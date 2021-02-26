use crate::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "OutgoingMessageId::from_uuid")]
pub struct OutgoingMessageId(Uuid);

impl OutgoingMessageId {
    fn from_uuid(value: Uuid) -> Result<OutgoingMessageId, Failure> {
        return Ok(OutgoingMessageId(value));
    }

    pub fn new() -> OutgoingMessageId {
        return OutgoingMessageId(Uuid::new_v4());
    }
}