use crate::market::common::error::ticker_parse_error::TickerParseError;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Ticker::from_string")]
pub struct Ticker(String);

impl Ticker {
    fn from_string(id: String) -> Result<Self, TickerParseError> {
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