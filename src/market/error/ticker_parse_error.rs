#[derive(Error, Debug)]
pub enum TickerParseError {
    #[error("Fund id is empty")]
    Empty,
    #[error("Fund id contains invalid char")]
    InvalidValue,
}