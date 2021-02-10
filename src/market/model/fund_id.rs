use crate::market::model::ticker::Ticker;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "FundId::from_ticker")]
pub struct FundId(Ticker);

impl FundId {
    fn from_ticker(value: Ticker) -> Result<FundId, Failure> {
        return Ok(FundId(value));
    }
}