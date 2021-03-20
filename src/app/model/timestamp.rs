use crate::prelude::*;
use crate::app::model::datetime::DateTime;
use std::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "TimeStamp::from_u64")]
pub struct TimeStamp(u64);

impl TimeStamp {
    pub fn from_u64(value: u64) -> Result<TimeStamp, Failure> {
        return Ok(TimeStamp(value));
    }

    pub fn zero() -> TimeStamp {
        return TimeStamp(0);
    }

    pub fn now() -> TimeStamp {
        return DateTime::now().to_timestamp();
    }

    pub fn to_datetime(&self) -> Result<DateTime, Failure> {
        if self.0 == 0 {
            return crate::fail!("Timestamp is equals to zero");
        }
        let datetime = DateTime::from_timestamp(self.0);
        return Ok(datetime);
    }
}