use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Cik::from_string")]
pub struct Cik(String);

impl Cik {
    pub fn from_string(value: String) -> Result<Cik, Failure> {
        if value.len() == 0 {
            return crate::fail!("Cik cannot be empty");
        }
        for c in value.chars() {
            if !c.is_digit(10) {
                return crate::fail!("Cik contain invalid chars");
            }
        }
        return Ok(Cik(value));
    }
}