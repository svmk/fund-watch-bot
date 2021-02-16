use crate::market::common::model::historical_price::HistoricalPrice;
use crate::market::common::model::historical_volume::HistoricalVolume;
use crate::app::model::datetime::DateTime;

#[derive(new, Debug, Clone)]
pub struct HistoricalCandleStick {
    timestamp: DateTime,
    open: HistoricalPrice,
    close: HistoricalPrice,
    low: HistoricalPrice,
    high: HistoricalPrice,
    volume: HistoricalVolume,
}