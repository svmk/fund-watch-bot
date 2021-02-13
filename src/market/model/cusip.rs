use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Cusip::from_string")]
pub struct Cusip(String);

impl Cusip {
    pub fn from_string(value: String) -> Result<Cusip, Failure> {
        if value.len() != 6 {
            return Err(Failure::msg("Cusip lenght is not equal 6"));
        }
        for c in value.chars() {
            if !c.is_digit(10) {
                return Err(Failure::msg("Invalid cusip format"));
            }
        }
        return Ok(Cusip(value));
    }
}