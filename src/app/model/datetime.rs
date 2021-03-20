use crate::{app::model::day::Day, market::market_data::model::time_frame::TimeFrame, prelude::*};
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
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, derive_more::Display)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn now() -> DateTime {
        return DateTime(Utc::now());
    }

    pub fn ymd_start_day(year: Year, month: Month, day: Day) -> DateTime {
        let datetime = Utc.ymd(year.to_i32(), month.to_u32(), day.to_u32()).and_hms(0, 0, 0);
        return DateTime(datetime);
    }

    pub fn ymd_end_day(year: Year, month: Month, day: Day) -> DateTime {
        let datetime = Utc.ymd(year.to_i32(), month.to_u32(), day.to_u32()).and_hms(23, 59, 59);
        return DateTime(datetime);
    }

    pub fn from_year_start_day(year: Year) -> DateTime {
        return DateTime::ymd_start_day(year, Month::Jan, Day::DAY_1);
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

    pub fn prev_years(&self, years: u16) -> Result<DateTime, Failure> {
        let prev_year = self.0.with_year(self.0.year() - years as i32);
        let prev_year = match prev_year {
            Some(prev_year) => prev_year,
            None => {
                return crate::fail!("Unable to find prev year `{}`", self.get_year());
            },
        };
        let prev_year = DateTime(prev_year);
        return Ok(prev_year);
    }

    pub fn prev_timeframe(&self, time_frame: TimeFrame) -> Result<DateTime, Failure> {
        match time_frame {
            TimeFrame::Year => {
                return self.prev_years(1);
            },
            TimeFrame::Month => {
                return Ok(self.sub_days(31));
            },
            TimeFrame::Day => {
                return Ok(self.sub_days(1));
            },
        }
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

impl FromStr for DateTime {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = ChronoDateTime::parse_from_rfc3339(s)?;
        let value = value.with_timezone(&Utc{});
        let value = DateTime(value);
        return Ok(value);       
    }
}