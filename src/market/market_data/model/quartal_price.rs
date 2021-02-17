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
}

impl QuartalPrice {
    pub fn new(id: QuartalPriceId, candlestick: HistoricalCandleStick) -> QuartalPrice {
        return QuartalPrice {
            id,
            candlestick,
            daily_prices: Vec::new(),
        };
    }

    pub fn is_candlestick_equals(&self, candelstick: &HistoricalCandleStick) -> bool {
        return &self.candlestick == candelstick;
    }
}

impl Entity<QuartalPriceId> for QuartalPrice {
    fn get_entity_id(&self) -> &QuartalPriceId {
        return &self.id;
    }
}