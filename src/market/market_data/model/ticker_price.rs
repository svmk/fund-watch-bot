use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::common::model::candlestick::CandleStick;

#[derive(Debug)]
pub struct TickerPrice {
    ticker: Ticker,
    candlestick: CandleStick,
    prices: Vec<QuartalPriceId>,
}

impl TickerPrice {
    pub fn new(ticker: Ticker, candlestick: CandleStick) -> TickerPrice {
        return TickerPrice {
            ticker,
            candlestick,
            prices: Vec::new(),
        };
    }
}