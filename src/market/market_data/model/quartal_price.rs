use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::market::common::model::candlestick::CandleStick;
#[derive(Debug)]
pub struct QuartalPrice {
    id: QuartalPriceId,
    candlestick: CandleStick,
    prices: Vec<DayPriceId>,
}

impl QuartalPrice {
    pub fn new(id: QuartalPriceId, candlestick: CandleStick) -> QuartalPrice {
        return QuartalPrice {
            id,
            candlestick,
            prices: Vec::new(),
        };
    }
}