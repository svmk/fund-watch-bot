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