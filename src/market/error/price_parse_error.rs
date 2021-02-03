use crate::prelude::*;

#[derive(Error, Debug)]
pub enum PriceParseError {
    #[error("Unable to parse price: {0}")]
    Parse(#[from] Failure),
    #[error("Price is negative")]
    Negative,
    #[error("Unable to parse price: Invalid value")]
    Invalid,
}