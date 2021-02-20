use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Share(u64);

impl Share {
    pub fn zero() -> Share {
        return Share(0);
    }
    
    pub fn add(self, other: Share) -> Share {
        return Share(self.0 + other.0);
    }

    pub fn into_f64(self) -> f64 {
        return self.0 as f64;
    }
}

impl FromStr for Share {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u64::from_str(s)?;
        return Ok(Share(value));
    }
}