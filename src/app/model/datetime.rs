use chrono::{DateTime as ChronoDateTime};
use chrono::offset::Utc;
use chrono::naive::NaiveDateTime;
use crate::app::model::timestamp::TimeStamp;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, derive_more::Display)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn now() -> DateTime {
        return DateTime(Utc::now());
    }

    pub fn from_timestamp(timestamp: u64) -> DateTime {
        let datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let datetime = ChronoDateTime::from_utc(datetime, Utc{});
        return DateTime(datetime);
    }

    pub fn to_timestamp(&self) -> TimeStamp {
        return TimeStamp::from_u64(self.0.timestamp() as u64).unwrap();
    }
}