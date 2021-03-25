use crate::prelude::*;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    Start,
    Funds,
    Subscriptions,
    Unknown,
}

impl Command {
    const COMMAND_START: &'static str = "start";
    const COMMAND_FUNDS: &'static str = "funds";
    const COMMAND_SUBSCRIPTIONS: &'static str = "subscriptions";
    const COMMAND_UNKNOWN: &'static str = "unknown";
}

impl FromStr for Command {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            Self::COMMAND_START => {
                Command::Start
            },
            Self::COMMAND_FUNDS => {
                Command::Funds
            },
            Self::COMMAND_SUBSCRIPTIONS => {
                Command::Subscriptions
            },
            _ => {
                Command::Unknown
            },
        };
        return Ok(value);
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Command::Start => {
                write!(f, "{}", Self::COMMAND_START)
            },
            &Command::Funds => {
                write!(f, "{}", Self::COMMAND_FUNDS)
            },
            &Command::Subscriptions => {
                write!(f, "{}", Self::COMMAND_SUBSCRIPTIONS)
            },
            &Command::Unknown => {
                write!(f, "{}", Self::COMMAND_UNKNOWN)
            },
        }
    }
}