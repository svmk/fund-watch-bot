use crate::prelude::*;
use crate::app::model::year::Year;
use chrono::Date as ChronoDate;
use chrono::offset::Utc;
use chrono::Datelike;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

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