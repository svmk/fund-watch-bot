use crate::prelude::*;
use crate::market::error::price_parse_error::PriceParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Price(f64);

impl FromStr for Price {
    type Err = PriceParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = f64::from_str(s).map_err(Failure::msg)?;
        if value < 0.0 {
            return Err(PriceParseError::Negative);
        }
        if value.is_nan() {
            return Err(PriceParseError::Invalid);
        }
        if value.is_infinite() {
            return Err(PriceParseError::Invalid);
        }
        return Ok(Price(value));
    }
}