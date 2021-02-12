use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Cik::from_u32")]
pub struct Cik(u32);

impl Cik {
    pub fn from_u32(value: u32) -> Result<Cik, Failure> {
        unimplemented!("Cik value example: 0001730817");
        if value == 0 {
            return Err(Failure::msg("Cik value is zero"));
        }
        return Ok(Cik(value));
    }
}