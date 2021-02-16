use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "ActualVolume::from_f64")]
pub struct ActualVolume(f64);

impl ActualVolume {
    fn from_f64(value: f64) -> Result<ActualVolume, Failure> {
        return Ok(ActualVolume(value));
    }

    pub fn into_f64(&self) -> f64 {
        return self.0;
    }
}