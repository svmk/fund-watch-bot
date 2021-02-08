use crate::market::error::ticker_parse_error::TickerParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Ticker(String);

impl FromStr for Ticker {
    type Err = TickerParseError;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        if id.is_empty() {
            return Err(TickerParseError::Empty);
        }
        if id.to_uppercase() != id {
            return Err(TickerParseError::InvalidValue);
        }
        let id = id.to_string();
        return Ok(Ticker(id));
    }
}