use crate::error::failure::Failure;
use crate::market::common::model::company_id::CompanyId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CandlestickFetchError {
    #[error("Company `{0}` source not available")]
    CompanyNotAvailable(CompanyId),
    #[error("{0}")]
    Custom(#[from] Failure),
}

impl CandlestickFetchError {
    pub fn is_ticker_not_available(&self) -> bool {
        match self {
            CandlestickFetchError::CompanyNotAvailable(..) => true,
            _ => false,
        }
    }
}