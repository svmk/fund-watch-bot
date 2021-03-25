use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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