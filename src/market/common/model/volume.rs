use crate::prelude::*;


#[derive(Debug, Clone, PartialEq, PartialOrd, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Volume::from_f64")]
pub struct Volume(f64);

impl Volume {
    fn from_f64(value: f64) -> Result<Volume, Failure> {
        return Ok(Volume(value));
    }
}