use std::str::FromStr;
use crate::prelude::*;

#[derive(Debug)]
pub enum InvestmentDiscretion {
    Sole,
}

impl FromStr for InvestmentDiscretion {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SOLE" => {
                return Ok(InvestmentDiscretion::Sole);
            },
            _ => {
                return Err(Failure::msg(format!("Wrong investment discretion value: `{}`", s)));
            },
        }
    }
}