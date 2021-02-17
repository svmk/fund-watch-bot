use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::market::common::model::historical_candlestick::HistoricalCandleStick;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuartalPrice {
    #[serde(rename="id")]
    id: QuartalPriceId,
    #[serde(rename="candlestick")]
    candlestick: HistoricalCandleStick,
    #[serde(rename="daily_prices")]
    daily_prices: Vec<DayPriceId>,
    #[serde(rename="incomplete_daily_prices")]
    incomplete_daily_prices: Vec<DayPriceId>,
}

impl QuartalPrice {
    pub fn new(id: QuartalPriceId, candlestick: HistoricalCandleStick) -> QuartalPrice {
        return QuartalPrice {
            id,
            candlestick,
            daily_prices: Vec::new(),
            incomplete_daily_prices: Vec::new(),
        };
    }

    pub fn is_candlestick_equals(&self, candelstick: &HistoricalCandleStick) -> bool {
        return &self.candlestick == candelstick;
    }

    pub fn contains_daily_price(&self, daily_price: &DayPriceId) -> bool {
        return self.daily_prices.binary_search(daily_price).is_ok();
    }

    pub fn push_daily_price_once(&mut self, daily_price: DayPriceId) {
        if !self.contains_daily_price(&daily_price) {
            self.daily_prices.push(daily_price);
        }
        self.daily_prices.sort();
    }

    pub fn contains_incomplete_daily_price(&self, incomplete_daily_price: &DayPriceId) -> bool {
        return self.incomplete_daily_prices.binary_search(incomplete_daily_price).is_ok();
    }
    
    pub fn push_incomplete_daily_price_once(&mut self, incomplete_daily_price: DayPriceId) {
        if !self.contains_incomplete_daily_price(&incomplete_daily_price) {
            self.incomplete_daily_prices.push(incomplete_daily_price);
        }
        self.incomplete_daily_prices.sort();
    }
}

impl Entity<QuartalPriceId> for QuartalPrice {
    fn get_entity_id(&self) -> &QuartalPriceId {
        return &self.id;
    }
}