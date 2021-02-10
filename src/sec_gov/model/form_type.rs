use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "FormType::from_string")]
pub struct FormType(String);

impl FormType {
    pub fn from_string(value: String) -> Result<FormType, Failure> {
        if value.is_empty() {
            return Err(Failure::msg("Form type is empty"));
        }
        return Ok(FormType(value));
    }
}