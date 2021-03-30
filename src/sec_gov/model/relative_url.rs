
use crate::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "RelativeUrl::from_string")]
pub struct RelativeUrl(String);

impl RelativeUrl {
    pub fn from_string(value: String) -> Result<RelativeUrl, Failure> {
        if value.is_empty() {
            return crate::fail!("Edgar path is empty");
        }
        return Ok(RelativeUrl(value));
    }

    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}
