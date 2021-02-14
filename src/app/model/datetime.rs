use chrono::{Date, DateTime as ChronoDateTime};
use chrono::offset::Utc;
use chrono::naive::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, derive_more::Display)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn from_timestamp(timestamp: u64) -> DateTime {
        let datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let datetime = ChronoDateTime::from_utc(datetime, Utc{});
        return DateTime(datetime);
    }
}