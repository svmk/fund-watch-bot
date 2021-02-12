use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "CompanyName::from_string")]
pub struct CompanyName(String);

impl CompanyName {
    pub fn from_string(id: String) -> Result<Self, Failure> {
        if id.trim() != id {
            return Err(Failure::msg("Company name has leading spaces"));
        }
        if id.is_empty() {
            return Err(Failure::msg("Company name cannot be empty"));
        }
        if id.to_uppercase() != id {
            return Err(Failure::msg("Company name must be uppercase"));
        }
        let id = id.to_string();
        return Ok(CompanyName(id));
    }
}