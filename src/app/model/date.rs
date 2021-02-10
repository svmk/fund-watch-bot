use chrono::Date as ChronoDate;
use chrono::offset::Utc;
use chrono::Datelike;

#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub struct Date(ChronoDate<Utc>);

impl Date {
    pub fn today() -> Date {
        return Date(Utc::today());
    }

    pub fn get_year(&self) -> i32 {
        return self.0.year();
    }
}