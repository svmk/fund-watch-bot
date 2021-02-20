use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "Share::from_u64")]
pub struct Share(u64);

impl Share {
    pub fn zero() -> Share {
        return Share(0);
    }
    
    pub fn add(self, other: Share) -> Share {
        return Share(self.0 + other.0);
    }

    pub fn into_f64(self) -> f64 {
        return self.0 as f64;
    }

    fn from_u64(value: u64) -> Result<Share, Failure> {
        return Ok(Share(value));
    }
}