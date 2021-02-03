use crate::market::model::ticker::Ticker;
use crate::market::error::ticker_parse_error::TickerParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct FundId(Ticker);

impl FromStr for FundId {
    type Err = TickerParseError;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let ticker = Ticker::from_str(id)?;
        let fund_id = FundId(ticker);
        return Ok(fund_id);
    }
}