use crate::prelude::*;
use std::fmt;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    pub fn from_u32(month: u32) -> Result<Month, Failure> {
        let month = match month {
            1 => Month::Jan,
            2 => Month::Feb,
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            _ => {
                return Err(Failure::msg(format!("Unknown month value `{}`", month)));
            },
        };
        return Ok(month);
    }

    pub fn to_u32(&self) -> u32 {
        return match self {
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dec => 12,
        };
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Month::Jan => {
                write!(f, "Jan")
            },
            Month::Feb => {
                write!(f, "Feb")
            },
            Month::Mar => {
                write!(f, "Mar")
            },
            Month::Apr => {
                write!(f, "Apr")
            },
            Month::May => {
                write!(f, "May")
            },
            Month::Jun => {
                write!(f, "Jun")
            },
            Month::Jul => {
                write!(f, "Jul")
            },
            Month::Aug => {
                write!(f, "Aug")
            },
            Month::Sep => {
                write!(f, "Sep")
            },
            Month::Oct => {
                write!(f, "Oct")
            },
            Month::Nov => {
                write!(f, "Nov")
            },
            Month::Dec => {
                write!(f, "Dec")
            },
        }
    }
}

impl Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let value = self.to_u32();
        return value.serialize(serializer);
    }
}

impl <'de>Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let value = u32::deserialize(deserializer)?;
            let value = Month::from_u32(value).map_err(serde::de::Error::custom)?;
            return Ok(value);
        }
}