use crate::market::market_data::model::split::Split;
use crate::yahoo_finance::model::chart::chart_split::ChartSplit;
use crate::yahoo_finance::model::chart::chart_dividiend::ChartDividend;
use crate::app::model::timestamp::TimeStamp;
use crate::market::common::model::actual_price::ActualPrice;
use crate::market::common::model::actual_volume::ActualVolume;
use crate::market::common::model::actual_candlestick::ActualCandleStick;
use std::collections::HashMap;
use itertools::izip;

#[derive(Debug, serde_query::Deserialize)]
pub struct ChartResponse {
    #[query(".events.splits")]
    splits: HashMap<String, ChartSplit>,
    #[query(".events.dividends")]
    dividends: HashMap<String, ChartDividend>,
    #[query(".timestamp")]
    timestamps: Vec<TimeStamp>,
    #[query(".indicators.[0].quote.open")]
    open: Vec<ActualPrice>,
    #[query(".indicators.[0].quote.close")]
    close: Vec<ActualPrice>,
    #[query(".indicators.[0].quote.high")]
    high: Vec<ActualPrice>,
    #[query(".indicators.[0].quote.low")]
    low: Vec<ActualPrice>,
    #[query(".indicators.[0].quote.volume")]
    volume: Vec<ActualVolume>,
}

impl ChartResponse {
    pub fn get_splits(&self) -> Vec<Split> {
        let mut splits: Vec<Split> = self
            .splits
            .values()
            .map(ChartSplit::create_split)
            .collect();
        splits.sort_by_key(|item| {
            return item.get_datetime().clone();
        });
        return splits;
    }

    pub fn get_candlesticks(&self) -> Vec<ActualCandleStick> {
        let iterator = izip!(
            self.timestamps.iter(),
            self.open.iter(),
            self.close.iter(),
            self.high.iter(),
            self.low.iter(),
            self.volume.iter(),
        );
        let mut result = Vec::new();
        for item in iterator {
            let (timestamp, open, close, high, low, volume) = item;
            let candlestick = ActualCandleStick::new(
                timestamp.to_datetime(),
                open.clone(),
                close.clone(),
                low.clone(),
                high.clone(),
                volume.clone(),
            );
            result.push(candlestick);
        }
        return result;
    }
}