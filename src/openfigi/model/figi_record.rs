use crate::market::common::model::ticker::Ticker;
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigiRecord {
    #[serde(rename = "ticker")]
    ticker: Option<Ticker>,
    #[serde(flatten)]
    values: HashMap<String, Option<String>>,
}

impl FigiRecord {
    pub fn get_ticker(&self) -> Result<Ticker, Failure> {
        if let Some(ref ticker) = self.ticker {
            return Ok(ticker.clone());
        }
        return Err(Failure::msg("Ticker not found in figi record"));
    }

    pub fn get_opt_ticker(&self) -> Option<Ticker> {
        if let Some(ref ticker) = self.ticker {
            return Some(ticker.clone());
        }
        return None;
    }
}