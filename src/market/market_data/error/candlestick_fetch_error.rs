use crate::error::failure::Failure;
use crate::market::common::model::ticker::Ticker;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CandlestickFetchError {
    #[error("Ticker `{0}` source not available")]
    TickerNotAvailable(Ticker),
    #[error("{0}")]
    Custom(#[from] Failure),
}

impl CandlestickFetchError {
    pub fn is_ticker_not_available(&self) -> bool {
        match self {
            CandlestickFetchError::TickerNotAvailable(..) => true,
            _ => false,
        }
    }
}