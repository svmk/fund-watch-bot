use crate::prelude::*;
use crate::market::market_data::model::split::Split;
use crate::yahoo_finance::model::chart::chart_split::ChartSplit;
use crate::yahoo_finance::model::chart::chart_dividiend::ChartDividend;
use crate::app::model::timestamp::TimeStamp;
use crate::market::common::model::actual_price::ActualPrice;
use crate::market::common::model::actual_volume::ActualVolume;
use crate::market::common::model::actual_candlestick::ActualCandleStick;
use std::collections::HashMap;
use itertools::izip;


#[derive(Debug, Deserialize)]
struct Events {
    #[serde(rename="splits")]
    pub splits: Option<HashMap<String, ChartSplit>>,
    #[serde(rename="dividends")]
    pub dividends: Option<HashMap<String, ChartDividend>>,
}

#[derive(Debug, Deserialize)]
struct Candlestick {
    #[serde(rename="open")]
    pub open: Vec<ActualPrice>,
    #[serde(rename="close")]
    pub close: Vec<ActualPrice>,
    #[serde(rename="high")]
    pub high: Vec<ActualPrice>,
    #[serde(rename="low")]
    pub low: Vec<ActualPrice>,
    #[serde(rename="volume")]
    pub volume: Vec<ActualVolume>,
}

#[derive(Debug, Deserialize)]
struct Indicators {
    #[serde(rename="quote")]
    pub quote: Vec<Candlestick>,
}

impl Indicators {
    pub fn get_candlestick(&self) -> Result<&Candlestick, Failure> {
        if let Some(candlestick) = self.quote.first() {
            return Ok(candlestick);
        }
        return crate::fail!("Unable to get candlestick from yahoo chart");
    }
}

#[derive(Debug, Deserialize)]
pub struct ChartResponse {
    #[serde(rename="events")]
    events: Option<Events>,
    #[serde(rename="indicators")]
    indicators: Indicators,
    #[serde(rename="timestamp")]
    timestamps: Vec<TimeStamp>,
}

impl ChartResponse {
    pub fn get_splits(&self) -> Vec<Split> {
        let splits = self.events.as_ref().and_then(|events| {
            return events.splits.as_ref();
        });
        if let Some(ref splits) = splits {
            let mut splits: Vec<Split> = splits
            .values()
            .map(ChartSplit::create_split)
            .collect();
            splits.sort_by_key(|item| {
                return item.get_datetime().clone();
            });
            return splits;
        }
        return Vec::new();
    }

    pub fn get_candlesticks(&self) -> Result<Vec<ActualCandleStick>, Failure> {
        let candlestick = self.indicators.get_candlestick()?;
        let iterator = izip!(
            self.timestamps.iter(),
            candlestick.open.iter(),
            candlestick.close.iter(),
            candlestick.high.iter(),
            candlestick.low.iter(),
            candlestick.volume.iter(),
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
        return Ok(result);
    }
}