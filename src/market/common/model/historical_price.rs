use crate::market::common::error::price_parse_error::PriceParseError;
use crate::market::common::model::price::Price;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "PriceParseError", load_fn = "HistoricalPrice::from_f64")]
pub struct HistoricalPrice(f64);

impl HistoricalPrice {
    pub fn from_price(value: Price) -> HistoricalPrice {
        return HistoricalPrice(value.into_f64());
    }
    
    fn from_f64(value: f64) -> Result<HistoricalPrice, PriceParseError> {
        if value < 0.0 {
            return Err(PriceParseError::Negative);
        }
        if value.is_nan() {
            return Err(PriceParseError::Invalid);
        }
        if value.is_infinite() {
            return Err(PriceParseError::Invalid);
        }
        return Ok(HistoricalPrice(value));
    }
}