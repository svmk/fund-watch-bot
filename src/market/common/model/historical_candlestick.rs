use crate::market::common::model::historical_price::HistoricalPrice;
use crate::market::common::model::historical_volume::HistoricalVolume;
use crate::app::model::datetime::DateTime;

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalCandleStick {
    #[serde(rename="timestamp")]
    timestamp: DateTime,
    #[serde(rename="open")]
    open: HistoricalPrice,
    #[serde(rename="close")]
    close: HistoricalPrice,
    #[serde(rename="low")]
    low: HistoricalPrice,
    #[serde(rename="high")]
    high: HistoricalPrice,
    #[serde(rename="volume")]
    volume: HistoricalVolume,
}

impl HistoricalCandleStick {
    pub fn get_timestamp(&self) -> &DateTime {
        return &self.timestamp;
    }

    pub fn get_close(&self) -> &HistoricalPrice {
        return &self.close;
    }
}