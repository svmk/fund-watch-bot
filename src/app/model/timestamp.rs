use crate::prelude::*;
use crate::app::model::datetime::DateTime;
use std::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "TimeStamp::from_u64")]
pub struct TimeStamp(u64);

impl TimeStamp {
    fn from_u64(value: u64) -> Result<TimeStamp, Failure> {
        return Ok(TimeStamp(value));
    }

    pub fn to_datetime(&self) -> DateTime {
        return DateTime::from_timestamp(self.0);
    }
}