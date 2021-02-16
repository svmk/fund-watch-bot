use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::market::common::model::historical_candlestick::HistoricalCandleStick;
#[derive(Debug)]
pub struct QuartalPrice {
    id: QuartalPriceId,
    candlestick: HistoricalCandleStick,
    prices: Vec<DayPriceId>,
}

impl QuartalPrice {
    pub fn new(id: QuartalPriceId, candlestick: HistoricalCandleStick) -> QuartalPrice {
        return QuartalPrice {
            id,
            candlestick,
            prices: Vec::new(),
        };
    }

    pub fn is_candlestick_differ(&self, candelstick: &HistoricalCandleStick) -> bool {
        return &self.candlestick != candelstick;
    }
}