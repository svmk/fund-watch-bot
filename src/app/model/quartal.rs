use crate::app::model::month::Month;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
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