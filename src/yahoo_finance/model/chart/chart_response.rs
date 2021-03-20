use crate::prelude::*;
use crate::market::market_data::model::split::Split;
use crate::market::market_data::model::chart_period::ChartPeriod;
use crate::market::market_data::model::actual_chart_period::ActualChartPeriod;
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
    #[serde(rename="open", default = "Default::default")]
    pub open: Vec<Option<ActualPrice>>,
    #[serde(rename="close", default = "Default::default")]
    pub close: Vec<Option<ActualPrice>>,
    #[serde(rename="high", default = "Default::default")]
    pub high: Vec<Option<ActualPrice>>,
    #[serde(rename="low", default = "Default::default")]
    pub low: Vec<Option<ActualPrice>>,
    #[serde(rename="volume", default = "Default::default")]
    pub volume: Vec<Option<ActualVolume>>,
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
    #[serde(rename="timestamp", default = "Default::default")]
    timestamps: Vec<Option<TimeStamp>>,
}

impl ChartResponse {
    pub fn get_splits(&self) -> Result<Vec<Split>, Failure> {
        let splits = self.events.as_ref().and_then(|events| {
            return events.splits.as_ref();
        });
        if let Some(ref splits) = splits {
            let splits_iterator = splits
            .values()
            .map(ChartSplit::create_split);
            let mut splits_result = Vec::new();
            for split in splits_iterator {
                let split = split?;
                splits_result.push(split);
            }
            splits_result.sort_by_key(|item| {
                return item.get_datetime().clone();
            });
            return Ok(splits_result);
        }
        return Ok(Vec::new());
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
        let iterator = iterator
            .filter_map(|(timestamp, open, close, high, low, volume)| {
                let timestamp = timestamp.as_ref()?;
                let open = open.as_ref()?;
                let close = close.as_ref()?;
                let high = high.as_ref()?;
                let low = low.as_ref()?;
                let volume = volume.as_ref()?;
                return Some((timestamp, open, close, high, low, volume));
            });
        let mut result = Vec::new();
        for item in iterator {
            let (timestamp, open, close, high, low, volume) = item;
            let candlestick = ActualCandleStick::new(
                timestamp.to_datetime()?,
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

    pub fn get_chart_period(&self) -> Result<ActualChartPeriod, Failure> {
        let first_timestamp = self
            .timestamps
            .iter()
            .filter_map(|item| {
                return item.as_ref();
            })
            .nth(0);
        let last_timestamp = self
            .timestamps
            .iter()
            .rev()
            .filter_map(|item| {
                return item.as_ref();
            })
            .nth(0);
        match (first_timestamp, last_timestamp) {
            (Some(first_timestamp), Some(last_timestamp)) => {
                let first_timestamp = first_timestamp.to_datetime()?;
                let last_timestamp = last_timestamp.to_datetime()?;
                let chart_period = ChartPeriod::new(first_timestamp, last_timestamp);
                let chart_period = ActualChartPeriod::new(chart_period);
                return Ok(chart_period);
            },
            _ => {
                return Ok(ActualChartPeriod::new_uncached());
            }
        }
    }
}