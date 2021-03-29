use std::str::FromStr;
use crate::prelude::*;

#[derive(Debug)]
/// https://fundapps.zendesk.com/hc/en-us/articles/360038126592-Notes-on-Various-13F-Portfolio-Properties
pub enum InvestmentDiscretion {
    Sole,
    Defund,
    Otr,
}

impl FromStr for InvestmentDiscretion {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SOLE" => {
                return Ok(InvestmentDiscretion::Sole);
            },
            "DFND" => {
                return Ok(InvestmentDiscretion::Defund);
            },
            "OTR" => {
                return Ok(InvestmentDiscretion::Otr);
            },
            _ => {
                return Err(Failure::msg(format!("Wrong investment discretion value: `{}`", s)));
            },
        }
    }
}