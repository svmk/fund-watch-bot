use crate::market::common::model::candlestick::CandleStick;
use crate::market::market_data::model::day_price_id::DayPriceId;
#[derive(Debug)]
pub struct DayPrice {
    id: DayPriceId,
    candlestick: CandleStick,
}