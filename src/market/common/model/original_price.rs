use crate::market::common::error::price_parse_error::PriceParseError;
use crate::market::common::model::price::Price;

#[derive(Debug, Clone, PartialEq, PartialOrd, ValueObject)]
#[value_object(error_type = "PriceParseError", load_fn = "OriginalPrice::from_f64")]
pub struct OriginalPrice(f64);

impl OriginalPrice {
    pub fn from_price(value: Price) -> OriginalPrice {
        return OriginalPrice(value.into_f64());
    }

    pub fn zero() -> OriginalPrice {
        return OriginalPrice(0.0);
    }

    pub fn max(self, other: &Self) -> OriginalPrice {
        return OriginalPrice(self.0.max(other.0));
    }

    pub fn min(self, other: &Self) -> OriginalPrice {
        return OriginalPrice(self.0.min(other.0));
    }

    pub fn sub(&self, other: &Self) -> OriginalPrice {
        let value = self.0 - other.0;
        assert!(value >= 0.0);
        return OriginalPrice(value);
    }
    
    pub fn from_f64(value: f64) -> Result<OriginalPrice, PriceParseError> {
        if value < 0.0 {
            return Err(PriceParseError::Negative);
        }
        if value.is_nan() {
            return Err(PriceParseError::Invalid);
        }
        if value.is_infinite() {
            return Err(PriceParseError::Invalid);
        }
        return Ok(OriginalPrice(value));
    }

    pub fn into_f64(&self) -> f64 {
        return self.0;
    }
}