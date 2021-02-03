use crate::prelude::*;
use crate::market::error::weight_parse_error::WeightParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Weight(f64);

impl FromStr for Weight {
    type Err = WeightParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = f64::from_str(s).map_err(Failure::msg)?;
        if value < 0.0 {
            return Err(WeightParseError::Negative);
        }
        if value.is_nan() {
            return Err(WeightParseError::Invalid);
        }
        if value.is_infinite() {
            return Err(WeightParseError::Invalid);
        }
        if value > 100.0 {
            return Err(WeightParseError::Over100);
        }
        return Ok(Weight(value));
    }
}