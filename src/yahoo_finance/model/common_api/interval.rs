use std::fmt;
#[derive(Debug)]
#[allow(dead_code)]
pub enum Interval {
    OneMinute,
    TwoMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    SixtyMinutes,
    NinetyMinutes,
    OneHour,
    OneDay,
    FiveDays,
    OneWeek,
    OneMonth,
    ThreeMonths,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::OneMinute => {
                write!(f, "1m")
            },
            Interval::TwoMinute => {
                write!(f, "2m")
            },
            Interval::FiveMinutes => {
                write!(f, "5m")
            },
            Interval::FifteenMinutes => {
                write!(f, "15m")
            },
            Interval::ThirtyMinutes => {
                write!(f, "30m")
            },
            Interval::SixtyMinutes => {
                write!(f, "60m")
            },
            Interval::NinetyMinutes => {
                write!(f, "90m")
            },
            Interval::OneHour => {
                write!(f, "1h")
            },
            Interval::OneDay => {
                write!(f, "1d")
            },
            Interval::FiveDays => {
                write!(f, "5d")
            },
            Interval::OneWeek => {
                write!(f, "1wk")
            },
            Interval::OneMonth => {
                write!(f, "1mo")
            },
            Interval::ThreeMonths => {
                write!(f, "3mo")
            },
        }       
    }
}