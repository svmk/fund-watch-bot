use chrono::{DateTime as ChronoDateTime, TimeZone};
use chrono::offset::Utc;
use chrono::naive::NaiveDateTime;
use crate::app::model::timestamp::TimeStamp;
use crate::app::model::year::Year;
use crate::app::model::month::Month;
use crate::app::model::date::Date;
use chrono::Datelike;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::Duration;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, derive_more::Display)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn now() -> DateTime {
        return DateTime(Utc::now());
    }

    pub fn ymd_start_day(year: i32, month: u32, day: u32) -> DateTime {
        let datetime = Utc.ymd(year, month, day).and_hms(0, 0, 0);
        return DateTime(datetime);
    }

    pub fn from_chrono_datetime(datetime: ChronoDateTime<Utc>) -> DateTime {
        return DateTime(datetime);
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

    pub fn to_date(&self) -> Date {
        let date = self.0.date();
        return Date::from_chrono_date(date);
    }

    pub fn add_days(&self, days: u32) -> DateTime {
        let days = Duration::days(days as i64);
        let datetime = self.0 + days;
        return DateTime(datetime);
    }

    pub fn sub_days(&self, days: u32) -> DateTime {
        let days = Duration::days(days as i64);
        let datetime = self.0 - days;
        return DateTime(datetime);
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
            return self.0.serialize(serializer);
        }   
}

impl <'de>Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let value: ChronoDateTime<Utc> = Deserialize::deserialize(deserializer)?;
            let value = DateTime(value);
            return Ok(value);
        }
}