use chrono::Date as ChronoDate;
use chrono::offset::Utc;

#[derive(Debug, Clone, PartialEq)]
pub struct Date(ChronoDate<Utc>);

impl Date {
    pub fn today() -> Date {
        return Date(Utc::today());
    }
}