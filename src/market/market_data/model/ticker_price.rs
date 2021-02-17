use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::split::Split;
use crate::market::market_data::model::split_rules::SplitRules;
use crate::market::common::model::historical_candlestick::HistoricalCandleStick;
use crate::market::common::model::actual_candlestick::ActualCandleStick;
use crate::app::model::datetime::DateTime;
use crate::repository::model::entity::Entity;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerPrice {
    #[serde(rename = "ticker")]
    ticker: Ticker,
    // candlestick: HistoricalCandleStick,
    #[serde(rename = "split_rules")]
    split_rules: SplitRules,
    #[serde(rename = "quartal_prices")]
    quartal_prices: Vec<QuartalPriceId>,
}

impl TickerPrice {
    pub fn new(
        ticker: Ticker, 
        // candlestick: HistoricalCandleStick,
    ) -> TickerPrice {
        return TickerPrice {
            ticker,
            // candlestick,
            split_rules: SplitRules::new(),
            quartal_prices: Vec::new(),
        };
    }

    pub fn can_add_split(&self, split: &Split) -> bool {
        return self.split_rules.can_add_split(split);
    }

    pub fn add_split(&mut self, split: Split) -> Result<(), Failure> {
        return self.split_rules.add_split(split);
    }

    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn calculate_historical_candlesticks(&self, actual_candlesticks: Vec<ActualCandleStick>) -> Result<Vec<HistoricalCandleStick>, Failure> {
        return self.split_rules.calculate_historical_candlesticks(actual_candlesticks);
    }

    pub fn create_quartal_price_id(&self, datetime: DateTime) -> QuartalPriceId {
        return QuartalPriceId::from_ticker_and_date(self.ticker.clone(), datetime);
    }
}

impl Entity<Ticker> for TickerPrice {
    fn get_entity_id(&self) -> &Ticker {
        return &self.ticker;
    }
}