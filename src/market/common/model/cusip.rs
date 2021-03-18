use crate::prelude::*;
use crate::repository::model::identity::Identity;
#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Cusip::from_string")]
pub struct Cusip(String);

impl Cusip {
    pub fn from_string(value: String) -> Result<Cusip, Failure> {
        if value.len() != 9 {
            return Err(Failure::msg("Cusip length is not equal 9"));
        }
        for c in value.chars() {
            let is_valid_char = c.is_digit(10) || c.is_uppercase() && c.is_alphabetic();
            if !is_valid_char {
                return Err(Failure::msg("Invalid cusip format"));
            }
        }
        return Ok(Cusip(value));
    }

    pub fn into_to_string(self) -> String {
        return self.0;
    }
}

impl Identity for Cusip {
    fn to_string(&self) -> String {
        return self.0.clone();
    }
}