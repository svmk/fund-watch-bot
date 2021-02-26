use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "ActionType::from_u16", from_str_derive = true)]
pub struct ActionType(u16);

impl ActionType {
    pub const FUND_LIST: ActionType = ActionType(1);
    fn from_u16(value: u16) -> Result<ActionType, Failure> {
        return Ok(ActionType(value));
    }
}