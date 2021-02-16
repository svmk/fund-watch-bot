use chrono::{DateTime as ChronoDateTime};
use chrono::offset::Utc;
use chrono::naive::NaiveDateTime;
use crate::app::model::timestamp::TimeStamp;
use crate::app::model::year::Year;
use crate::app::model::month::Month;
use chrono::Datelike;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, derive_more::Display)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn now() -> DateTime {
        return DateTime(Utc::now());
    }

    pub fn get_year(&self) -> Year {
        return Year::from_i32(self.0.year()).unwrap();
    }

    pub fn get_month(&self) -> Month {
        return Month::from_u32(self.0.month()).unwrap();
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