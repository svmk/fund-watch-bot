use crate::market::common::model::original_price::OriginalPrice;
use crate::market::common::model::original_volume::OriginalVolume;
use crate::app::model::datetime::DateTime;

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalCandleStick {
    #[serde(rename="timestamp")]
    timestamp: DateTime,
    #[serde(rename="open")]
    open: OriginalPrice,
    #[serde(rename="close")]
    close: OriginalPrice,
    #[serde(rename="low")]
    low: OriginalPrice,
    #[serde(rename="high")]
    high: OriginalPrice,
    #[serde(rename="volume")]
    volume: OriginalVolume,
}

impl OriginalCandleStick {
    pub fn group_from_iterator<'a>(timestamp: DateTime, iterator: impl Iterator<Item=&'a OriginalCandleStick>) -> Option<OriginalCandleStick> {
        let mut open = OriginalPrice::zero();
        let mut close = OriginalPrice::zero();
        let mut low = OriginalPrice::zero();
        let mut high = OriginalPrice::zero();
        let mut volume = OriginalVolume::zero();
        let mut is_first = true;
        for candlestick in iterator {
            if is_first {
                open = candlestick.get_open().clone();
                low = candlestick.get_low().clone();
                is_first = false;
            }
            close = candlestick.get_close().clone();
            low = low.min(candlestick.get_low());
            high = high.max(candlestick.get_high());
            volume = volume.sum(candlestick.get_volume());
        }
        if !is_first {
            let candlestick = OriginalCandleStick::new(
                timestamp,
                open,
                close,
                low,
                high,
                volume,
            );
            return Some(candlestick);
        }
        return None;
    }

    pub fn get_timestamp(&self) -> &DateTime {
        return &self.timestamp;
    }

    pub fn get_open(&self) -> &OriginalPrice {
        return &self.open;
    }

    pub fn get_close(&self) -> &OriginalPrice {
        return &self.close;
    }

    pub fn get_low(&self) -> &OriginalPrice {
        return &self.low;
    }
    
    pub fn get_high(&self) -> &OriginalPrice {
        return &self.high;
    }

    pub fn get_volume(&self) -> &OriginalVolume {
        return &self.volume;
    }
}