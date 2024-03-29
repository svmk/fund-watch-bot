use crate::app::model::year::Year;
use crate::app::model::quartal::Quartal;
use crate::app::model::date::Date;
use crate::app::model::datetime::DateTime;
use crate::prelude::*;
use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(new, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearQuartal {
    year: Year,
    quartal: Quartal,
}

impl YearQuartal {
    pub fn from_date(date: Date) -> YearQuartal {
        let year = date.get_year();
        let month = date.get_month();
        let quartal = Quartal::from_month(month);
        return YearQuartal {
            year,
            quartal,
        };
    }

    pub fn get_start(&self) -> DateTime {
        let (month, day) = self.quartal.start();
        return DateTime::ymd_start_day(self.year.clone(), month, day);
    }

    pub fn get_end(&self) -> DateTime {
        let (month, day) = self.quartal.end();
        return DateTime::ymd_end_day(self.year.clone(), month, day);
    }

    pub fn get_year(&self) -> &Year {
        return &self.year;
    }

    pub fn get_quartal(&self) -> &Quartal {
        return &self.quartal;
    }

    pub fn now() -> YearQuartal {
        let now = Date::today();
        let now = YearQuartal::from_date(now);
        return now;
    }

    pub fn next(&self) -> YearQuartal {
        let mut year = self.year.clone();
        let quartal = match &self.quartal {
            Quartal::Q1 => Quartal::Q2,
            Quartal::Q2 => Quartal::Q3,
            Quartal::Q3 => Quartal::Q4,
            Quartal::Q4 => {
                year = year.next();
                Quartal::Q1
            },
        };
        return YearQuartal {
            year,
            quartal,
        };
    }
}

impl fmt::Display for YearQuartal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}-{}", self.year, self.quartal);
    }
}

impl FromStr for YearQuartal {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err(Failure::msg("Invalid year quartal length"));
        }
        let year = &s[0..4];
        let year = Year::from_str(year)?;
        if &s[4..5] != "-" {
            return Err(Failure::msg("Invalid year quartal value"));
        }
        let quartal = &s[5..7];
        let quartal = Quartal::from_str(quartal)?;
        let value = YearQuartal {
            year,
            quartal,
        };
        return Ok(value);
    }
}

impl Serialize for YearQuartal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let value: String = format!("{}", self);
        return value.serialize(serializer);
    }
}

impl <'de>Deserialize<'de> for YearQuartal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?;
            let value = YearQuartal::from_str(&value).map_err(serde::de::Error::custom)?;
            return Ok(value);
        }
}