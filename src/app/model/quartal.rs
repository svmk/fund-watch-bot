use crate::app::model::month::Month;
use crate::app::model::day::Day;
use crate::prelude::*;
use std::fmt;
use std::str::FromStr;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quartal {
    #[serde(rename = "Q1")]
    Q1,
    #[serde(rename = "Q2")]
    Q2,
    #[serde(rename = "Q3")]
    Q3,
    #[serde(rename = "Q4")]
    Q4,
}

impl Quartal {
    pub fn from_month(month: Month) -> Quartal {
        match month {
            Month::Jan => Quartal::Q1,
            Month::Feb => Quartal::Q1,
            Month::Mar => Quartal::Q1,
            Month::Apr => Quartal::Q2,
            Month::May => Quartal::Q2,
            Month::Jun => Quartal::Q2,
            Month::Jul => Quartal::Q3,
            Month::Aug => Quartal::Q3,
            Month::Sep => Quartal::Q3,
            Month::Oct => Quartal::Q4,
            Month::Nov => Quartal::Q4,
            Month::Dec => Quartal::Q4,
        }
    }

    pub fn display_long(&self) -> &str {
        match self {
            Quartal::Q1 => {
                return "QTR1";
            },
            Quartal::Q2 => {
                return "QTR2";
            },
            Quartal::Q3 => {
                return "QTR3";
            },
            Quartal::Q4 => {
                return "QTR4";
            },
        }
    }

    pub fn start(&self) -> (Month, Day) {
        return match self {
            Quartal::Q1 => {
                (Month::Jan, Day::DAY_1)
            },
            Quartal::Q2 => {
                (Month::Apr, Day::DAY_1)
            },
            Quartal::Q3 => {
                (Month::Jul, Day::DAY_1)
            },
            Quartal::Q4 => {
                (Month::Oct, Day::DAY_1)
            },
        }
    }

    pub fn end(&self) -> (Month, Day) {
        return match self {
            Quartal::Q1 => {
                (Month::Mar, Day::DAY_31)
            },
            Quartal::Q2 => {
                (Month::Jun, Day::DAY_30)
            },
            Quartal::Q3 => {
                (Month::Sep, Day::DAY_30)
            },
            Quartal::Q4 => {
                (Month::Dec, Day::DAY_31)
            },
        }
    }
}

impl FromStr for Quartal {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "Q1" => Quartal::Q1,
            "Q2" => Quartal::Q2,
            "Q3" => Quartal::Q3,
            "Q4" => Quartal::Q4,
            _ => {
                return Err(Failure::msg(format!("Invalid quartal value: `{}`", s)));
            },
        };
        return Ok(value);
    }
}

impl fmt::Display for Quartal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quartal::Q1 => {
                write!(f, "Q1")
            },
            Quartal::Q2 => {
                write!(f, "Q2")
            },
            Quartal::Q3 => {
                write!(f, "Q3")
            },
            Quartal::Q4 => {
                write!(f, "Q4")
            },
        }
    }
}