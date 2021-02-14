use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, derive_more::Display)]
pub struct DateTime(ChronoDateTime<Utc>);