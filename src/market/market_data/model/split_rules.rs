use std::{convert::TryFrom, num::NonZeroU32};

use crate::market::{common::model::{actual_candlestick::ActualCandleStick, original_candlestick::{OriginalCandleStick}}, market_data::model::split_rule::SplitRule};
use crate::market::common::model::actual_price::ActualPrice;
use crate::market::common::model::actual_volume::ActualVolume;
use crate::market::market_data::model::split::Split;
use crate::market::common::model::original_volume::OriginalVolume;
use crate::market::common::model::original_price::OriginalPrice;
use crate::app::model::datetime::DateTime;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitRules {
    #[serde(rename = "split_rules")]
    split_rules: Vec<SplitRule>,
    #[serde(rename = "splits")]
    splits: Vec<Split>,
}

impl SplitRules {
    pub fn new() -> SplitRules {
        return SplitRules {
            split_rules: Vec::new(),
            splits: Vec::new(),
        };
    }

    pub fn can_add_split(&self, split: &Split) -> bool {
        if let Some(last_split) = self.splits.last() {
            return split.get_datetime() > last_split.get_datetime();
        }
        return true;
    }

    pub fn add_split(&mut self, split: Split) -> Result<(), Failure> {
        if !self.can_add_split(&split) {
            return Err(Failure::msg(format!("Split cannot be added for date {}", split.get_datetime())));
        }
        self.splits.push(split);
        self.update_split_rules()?;
        return Ok(());
    }

    fn update_split_rules(&mut self) -> Result<(), Failure> {
        let split_rules = self.generate_split_rules()?;
        self.split_rules = split_rules;
        return Ok(());
    }

    fn generate_split_rules(&self) -> Result<Vec<SplitRule>, Failure> {
        let mut previous_date: Option<DateTime> = None;
        let mut nominator = 1;
        let mut denominator = 1;
        let mut split_rules = Vec::new();
        for (split_index, split) in self.splits.iter().enumerate() {
            let split_date = split.get_datetime().clone();
            nominator = nominator * split.get_nominator().get();
            denominator = denominator * split.get_denominator().get();
            let is_last_split = (self.splits.len() - split_index) == 1;
            let started_at =  previous_date.clone();
            let ended_at = match is_last_split {
                true => {
                    None
                },
                false => {
                    Some(split_date.clone())
                },
            };
            let split_rule = SplitRule::new(
                started_at,
                ended_at,
                NonZeroU32::try_from(nominator)?,
                NonZeroU32::try_from(denominator)?,
            );
            split_rules.push(split_rule);
            previous_date = Some(split_date);
        }
        return Ok(split_rules);
    }

    fn calculate_original_candlestick(&self, actual_candlestick: &ActualCandleStick) -> Result<OriginalCandleStick, Failure> {
        let mut candlestick_open = actual_candlestick.get_open().into_f64();
        let mut candlestick_close = actual_candlestick.get_close().into_f64();
        let mut candlestick_high = actual_candlestick.get_high().into_f64();
        let mut candlestick_low = actual_candlestick.get_low().into_f64();
        let mut candlestick_volume = actual_candlestick.get_volume().into_f64();
        for split_rule in self.split_rules.iter().rev() {
            candlestick_open = split_rule.calculate_from_actual_to_original(candlestick_open);
            candlestick_close = split_rule.calculate_from_actual_to_original(candlestick_close);
            candlestick_high = split_rule.calculate_from_actual_to_original(candlestick_high);
            candlestick_low = split_rule.calculate_from_actual_to_original(candlestick_low);
            candlestick_volume = split_rule.calculate_from_actual_to_original(candlestick_volume);
        }
        let original_candlestick = OriginalCandleStick::new(
            actual_candlestick.get_timestamp().clone(),
            OriginalPrice::from_f64(candlestick_open)?,
            OriginalPrice::from_f64(candlestick_close)?,
            OriginalPrice::from_f64(candlestick_low)?,
            OriginalPrice::from_f64(candlestick_high)?,
            OriginalVolume::from_f64(candlestick_volume)?,
        );
        return Ok(original_candlestick);
    }

    pub fn calculate_actual_candlestick(&self, original_candlestick: &OriginalCandleStick) -> Result<ActualCandleStick, Failure> {
        let mut candlestick_open = original_candlestick.get_open().into_f64();
        let mut candlestick_close = original_candlestick.get_close().into_f64();
        let mut candlestick_high = original_candlestick.get_high().into_f64();
        let mut candlestick_low = original_candlestick.get_low().into_f64();
        let mut candlestick_volume = original_candlestick.get_volume().into_f64();
        for split_rule in self.split_rules.iter() {
            candlestick_open = split_rule.calculate_from_original_to_actual(candlestick_open);
            candlestick_close = split_rule.calculate_from_original_to_actual(candlestick_close);
            candlestick_high = split_rule.calculate_from_original_to_actual(candlestick_high);
            candlestick_low = split_rule.calculate_from_original_to_actual(candlestick_low);
            candlestick_volume = split_rule.calculate_from_original_to_actual(candlestick_volume);
        }
        let actual_candlestick = ActualCandleStick::new(
            original_candlestick.get_timestamp().clone(),
            ActualPrice::from_f64(candlestick_open)?,
            ActualPrice::from_f64(candlestick_close)?,
            ActualPrice::from_f64(candlestick_low)?,
            ActualPrice::from_f64(candlestick_high)?,
            ActualVolume::from_f64(candlestick_volume)?,
        );
        return Ok(actual_candlestick);
    }

    pub fn calculate_original_candlesticks(&self, actual_candlesticks: Vec<ActualCandleStick>) -> Result<Vec<OriginalCandleStick>, Failure> {
        let mut result = Vec::with_capacity(actual_candlesticks.len());
        for actual_candlestick in actual_candlesticks.iter() {
            let actual_candlestick = self.calculate_original_candlestick(actual_candlestick)?;
            result.push(actual_candlestick);
        }
        return Ok(result);
    }
}