use crate::market::common::model::historical_candlestick::HistoricalCandleStick;
use crate::app::model::datetime::DateTime;

#[derive(new, Debug)]
pub struct CandlestickReport {
    datetime: DateTime,
    daily: HistoricalCandleStick,
}