use crate::prelude::*;

#[derive(Error, Debug)]
pub enum WeightParseError {
    #[error("Unable to parse weight: {0}")]
    Parse(#[from] Failure),
    #[error("Weight is negative")]
    Negative,
    #[error("Unable to parse weight: Invalid value")]
    Invalid,
    #[error("Unable to parse weight: Over 100")]
    Over100,
}