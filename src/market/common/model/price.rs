use crate::market::common::error::price_parse_error::PriceParseError;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "PriceParseError", load_fn = "Price::from_f64")]
pub struct Price(f64);

impl Price {
    pub fn into_f64(self) -> f64 {
        return self.0;
    }
    
    pub fn from_f64(value: f64) -> Result<Price, PriceParseError> {
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