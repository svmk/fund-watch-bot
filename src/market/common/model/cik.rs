use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Cik::from_string")]
pub struct Cik(String);

impl Cik {
    pub fn from_string(value: String) -> Result<Cik, Failure> {
        const CIK_LENGTH: usize = 10;
        if value.len() == 0 {
            return crate::fail!("Cik cannot be empty");
        }
        if value.len() > CIK_LENGTH {
            return crate::fail!("Cik must be {} chars length or less", CIK_LENGTH);
        }
        for c in value.chars() {
            if !c.is_digit(CIK_LENGTH as u32) {
                return crate::fail!("Cik contain invalid chars");
            }
        }
        let zeroes = "0".repeat(CIK_LENGTH - value.len());
        let value = format!("{}{}", zeroes, value);
        return Ok(Cik(value));
    }
}