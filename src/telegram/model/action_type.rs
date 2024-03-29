use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "ActionType::from_u16", from_str_derive = true)]
pub struct ActionType(u16);

impl ActionType {
    pub const FUND_LIST: ActionType = ActionType(1);
    pub const SUBSCRIPTION_LIST: ActionType = ActionType(2);
    pub const FUND_INFO: ActionType = ActionType(3);
    pub const FUND_REPORT_LIST: ActionType = ActionType(4);
    pub const FUND_REPORT_INFO: ActionType = ActionType(5);
    pub const FUND_CHANGE_LIST: ActionType = ActionType(6);
    pub const FUND_CHANGE_INFO: ActionType = ActionType(7);
    fn from_u16(value: u16) -> Result<ActionType, Failure> {
        return Ok(ActionType(value));
    }
}