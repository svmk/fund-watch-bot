use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "MessageId::from_u32")]
pub struct MessageId(u32);

impl MessageId {
    pub fn from_u32(value: u32) -> Result<MessageId, Failure> {
        let value = MessageId(value);
        return Ok(value);
    }

    pub fn to_u32(self) -> u32 {
        return self.0;
    }
}