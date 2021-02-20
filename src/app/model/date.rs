use crate::prelude::*;
use crate::app::model::year::Year;
use crate::app::model::datetime::DateTime;
use crate::prelude::*;
use chrono::Date as ChronoDate;
use chrono::offset::Utc;
use chrono::Datelike;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, derive_more::Display)]
pub struct Date(ChronoDate<Utc>);

impl Date {
    pub fn from_chrono_date(date: ChronoDate<Utc>) -> Date {
        return Date(date);
    }
    
    pub fn today() -> Date {
        return Date(Utc::today());
    }

    pub fn get_year(&self) -> Year {
        return Year::from_i32(self.0.year()).unwrap();
    }

    pub fn parse_mdy(text: &str) -> Result<Date, Failure> {
        let date = NaiveDate::parse_from_str(text, "%m-%d-%Y")?;
        let date = ChronoDate::from_utc(date, Utc{});
        return Ok(Date(date));
    }

    pub fn next(&self) -> Date {
        let date = self.0.succ();
        return Date(date);
    }

    pub fn end_of_day(&self) -> DateTime {
        let datetime = self.0.and_hms(23, 59, 59);
        return DateTime::from_chrono_datetime(datetime);
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
            return self.0.naive_utc().serialize(serializer);
        }   
}

impl <'de>Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let value: NaiveDate = Deserialize::deserialize(deserializer)?;
            let value = ChronoDate::from_utc(value, Utc{});
            let value = Date(value);
            return Ok(value);
        }
}

impl FromStr for Date {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::from_str(s)?;
        let date = ChronoDate::from_utc(date, Utc{});
        let date = Date(date);
        return Ok(date);
    }
}