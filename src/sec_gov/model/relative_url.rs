
use crate::prelude::*;

#[derive(Debug, derive_more::Display)]
pub struct RelativeUrl(String);

impl RelativeUrl {
    pub fn new(value: String) -> RelativeUrl {
        return RelativeUrl(value);
    }

    pub fn from_string(value: String) -> Result<RelativeUrl, Failure> {
        if value.is_empty() {
            return Err(Failure::msg("Edgar path is empty"));
        }
        return Ok(RelativeUrl(value));
    }

    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}
