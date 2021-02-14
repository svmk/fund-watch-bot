use std::num::ParseFloatError;

#[derive(Error, Debug)]
pub enum PriceParseError {
    #[error("Unable to parse price: {0}")]
    Parse(#[from] ParseFloatError),
    #[error("Price is negative")]
    Negative,
    #[error("Unable to parse price: Invalid value")]
    Invalid,
}