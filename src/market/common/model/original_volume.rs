use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, ValueObject)]
#[value_object(error_type = "Failure", load_fn = "OriginalVolume::from_f64")]
pub struct OriginalVolume(f64);

impl OriginalVolume {
    pub fn from_f64(value: f64) -> Result<OriginalVolume, Failure> {
        return Ok(OriginalVolume(value));
    }

    pub fn zero() -> OriginalVolume {
        return OriginalVolume(0.0);
    }

    pub fn sum(self, other: &Self) -> OriginalVolume {
        return OriginalVolume(self.0 + other.0);
    }

    pub fn sub(&self, other: &Self) -> OriginalVolume {
        let value = self.0 - other.0;
        assert!(value >= 0.0);
        return OriginalVolume(value);
    }

    pub fn into_f64(&self) -> f64 {
        return self.0;
    }
}