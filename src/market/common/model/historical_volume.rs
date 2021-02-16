use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "HistoricalVolume::from_f64")]
pub struct HistoricalVolume(f64);

impl HistoricalVolume {
    fn from_f64(value: f64) -> Result<HistoricalVolume, Failure> {
        return Ok(HistoricalVolume(value));
    }
}