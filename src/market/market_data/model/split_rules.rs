use std::{convert::TryFrom, num::NonZeroU32};

use crate::market::market_data::model::split_rule::SplitRule;
use crate::market::market_data::model::split::Split;
use crate::market::common::model::actual_price::ActualPrice;
use crate::market::common::model::historical_price::HistoricalPrice;
use crate::app::model::datetime::DateTime;
use crate::prelude::*;

#[derive(Debug)]
pub struct SplitRules {
    split_rules: Vec<SplitRule>,
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

    pub fn calculate_historical_price(&self, datetime: &DateTime, actual_price: &ActualPrice) -> Result<HistoricalPrice, Failure> {
        let mut price = actual_price.clone().into_price();
        for split_rule in self.split_rules.iter().rev() {
            price = split_rule.calculate_historical_price(price)?;
            if split_rule.is_match_datetime(datetime) {
                break;
            }
        }
        let price = HistoricalPrice::from_price(price);
        return Ok(price);
    }
}