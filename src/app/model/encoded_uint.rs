use crate::prelude::*;
use std::str::FromStr;
use rand::random;
use radix_fmt::radix_36;
use std::fmt;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as SerdeError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncodedUint(u64);

impl EncodedUint {
    pub fn new() -> EncodedUint {
        let value: u64 = random();
        return EncodedUint(value);
    }
}

impl FromStr for EncodedUint {
    type Err = Failure;
    
    fn from_str(value: &str) -> Result<EncodedUint, Failure> {
        let value = u64::from_str_radix(value, 36)?;
        let value = EncodedUint(value);
        return Ok(value);
    }
}

impl fmt::Display for EncodedUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", radix_36(self.0))
    }
}

impl Serialize for EncodedUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
            let value = format!("{}", self);
            return value.serialize(serializer);
        }
}

impl <'de>Deserialize<'de> for EncodedUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?;
            let value = EncodedUint::from_str(&value).map_err(SerdeError::custom)?;
            return Ok(value);
        }
}