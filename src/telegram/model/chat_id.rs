use crate::prelude::*;
use crate::repository::model::identity::Identity;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "ChatId::from_i64")]
pub struct ChatId(i64);

impl ChatId {
    pub fn from_i64(value: i64) -> Result<ChatId, Failure> {
        return Ok(ChatId(value));
    } 
}

impl Identity for ChatId {
    fn to_string(&self) -> String {
        if self.0 < 0 {
            return format!("neg_{}", self.0);
        } else {
            return format!("{}", self.0);
        }
    }
}