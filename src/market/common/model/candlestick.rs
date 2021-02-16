use crate::market::common::model::historical_price::HistoricalPrice;
use crate::app::model::datetime::DateTime;

#[derive(new, Debug, Clone)]
pub struct CandleStick {
    timestamp: DateTime,
    low: HistoricalPrice,
    high: HistoricalPrice,
    open: HistoricalPrice,
    close: HistoricalPrice,
}