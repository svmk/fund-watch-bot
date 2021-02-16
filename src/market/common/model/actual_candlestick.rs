use crate::market::common::model::actual_price::ActualPrice;
use crate::market::common::model::actual_volume::ActualVolume;
use crate::app::model::datetime::DateTime;

#[derive(new, Debug, Clone)]
pub struct ActualCandleStick {
    timestamp: DateTime,
    open: ActualPrice,
    close: ActualPrice,
    low: ActualPrice,
    high: ActualPrice,
    volume: ActualVolume,
}

impl ActualCandleStick {
    pub fn get_timestamp(&self) -> &DateTime {
        return &self.timestamp;
    }

    pub fn get_open(&self) -> &ActualPrice {
        return &self.open;
    }

    pub fn get_close(&self) -> &ActualPrice {
        return &self.close;
    }

    pub fn get_low(&self) -> &ActualPrice {
        return &self.low;
    }

    pub fn get_high(&self) -> &ActualPrice {
        return &self.high;
    }

    pub fn get_volume(&self) -> &ActualVolume {
        return &self.volume;
    }   
}