use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Share(u64);

impl FromStr for Share {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u64::from_str(s)?;
        return Ok(Share(value));
    }
}