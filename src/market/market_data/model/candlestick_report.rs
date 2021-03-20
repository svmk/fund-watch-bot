use crate::market::common::model::original_candlestick::OriginalCandleStick;
use crate::market::common::model::actual_candlestick::ActualCandleStick;

#[derive(new, Debug)]
pub struct CandlestickReport {
    orignal: OriginalCandleStick,
    actual: ActualCandleStick,
}

impl CandlestickReport {
    pub fn get_orignal(&self) -> &OriginalCandleStick {
        return &self.orignal;
    }
}