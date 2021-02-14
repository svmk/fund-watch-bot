use crate::prelude::*;
use std::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "TimeStamp::from_u64")]
pub struct TimeStamp(u64);

impl TimeStamp {
    fn from_u64(value: u64) -> Result<TimeStamp, Failure> {
        return Ok(TimeStamp(value));
    }
}