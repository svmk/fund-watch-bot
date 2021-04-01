use crate::market::common::model::original_candlestick::OriginalCandleStick;


use std::{collections::HashMap, hash::Hash};
use itertools::Itertools;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart<Id> 
    where 
        Id: PartialEq + Eq,
        Id: Hash,
{
    candlesticks: HashMap<Id, OriginalCandleStick>,
}

impl <Id>Chart<Id> where 
    Id: PartialEq + Eq,
    Id: Hash,
{
    pub fn new() -> Chart<Id> {
        return Chart {
            candlesticks: HashMap::new(),
        }
    }

    pub fn update_chart_price(&mut self, id: &Id, new_candlestick: OriginalCandleStick) where Id: Clone {
        let _ = self.candlesticks.insert(id.clone(), new_candlestick);
    }

    pub fn need_update_chart_price(&self, id: &Id, new_candlestick: &OriginalCandleStick) -> bool where Id: Clone {
        if let Some(candlestick) = self.candlesticks.get(id) {
            return candlestick != new_candlestick;
        }
        return true;
    }

    pub fn iter_candlesticks(&self) -> impl Iterator<Item=&OriginalCandleStick> {
        return self.candlesticks.values().sorted_by_key(|candlestick| {
            return candlestick.get_timestamp();
        });
    }

    pub fn get(&self, id: &Id) -> Option<&OriginalCandleStick> {
        return self.candlesticks.get(id);
    }
}