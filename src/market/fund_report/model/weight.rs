use crate::prelude::*;
use crate::market::common::error::weight_parse_error::WeightParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Weight::from_f64")]
pub struct Weight(f64);

impl Weight {
    pub fn from_f64(value: f64) -> Result<Weight, WeightParseError> {
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