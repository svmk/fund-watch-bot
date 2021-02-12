use crate::prelude::*;
use chrono::Date as ChronoDate;
use chrono::offset::Utc;
use chrono::Datelike;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub struct Date(ChronoDate<Utc>);

impl Date {
    pub fn today() -> Date {
        return Date(Utc::today());
    }

    pub fn get_year(&self) -> i32 {
        return self.0.year();
    }

    pub fn parse_mdy(text: &str) -> Result<Date, Failure> {
        let date = NaiveDate::parse_from_str(text, "%m-%d-%Y")?;
        let date = ChronoDate::from_utc(date, Utc{});
        return Ok(Date(date));
    }
}