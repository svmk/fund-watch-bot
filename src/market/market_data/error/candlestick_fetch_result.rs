use crate::market::market_data::error::candlestick_fetch_error::CandlestickFetchError;

pub trait CandlestickFetchResult<T> {
    fn opt_available(self) -> Result<Option<T>, CandlestickFetchError>;
}

impl <T>CandlestickFetchResult<T> for Result<T, CandlestickFetchError> {
    fn opt_available(self) -> Result<Option<T>, CandlestickFetchError> {
        match self {
            Ok(result) => {
                Ok(Some(result))
            },
            Err(error) => {
                if error.is_ticker_not_available() {
                    Ok(None)
                } else {
                    Err(error)
                }
            },
        }
    }
}