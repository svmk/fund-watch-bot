use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Cik::from_string")]
pub struct Cik(String);

impl Cik {
    pub fn from_string(value: String) -> Result<Cik, Failure> {
        if value.len() != 10 {
            return Err(Failure::msg("Wrong cik format"));
        }
        return Ok(Cik(value));
    }
}