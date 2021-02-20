use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Start,
    Funds,
    Unknown,
}

impl Command {
    const COMMAND_START: &'static str = "start";
    const COMMAND_FUNDS: &'static str = "funds";
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
            _ => {
                Command::Unknown
            },
        };
        return Ok(value);
    }
}