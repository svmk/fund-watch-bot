use crate::market::model::ticker::Ticker;
use crate::market::model::share_change::ShareChange;
use crate::market::model::price_change::PriceChange;
use crate::market::model::weight_change::WeightChange;

#[derive(Debug, Clone)]
pub struct FundComponentChange {
    ticker: Ticker,
    share_change: Option<ShareChange>,
    price_change: Option<PriceChange>,
    weight_change: Option<WeightChange>,
}

impl FundComponentChange {
    pub fn new(ticker: Ticker) -> FundComponentChange {
        return FundComponentChange {
            ticker,
            share_change: None,
            price_change: None,
            weight_change: None,
        };
    }

    pub fn set_share_change(&mut self, value: ShareChange) {
        self.share_change = Some(value);
    }

    pub fn set_price_change(&mut self, value: PriceChange) {
        self.price_change = Some(value);
    }

    pub fn set_weight_change(&mut self, value: WeightChange) {
        self.weight_change = Some(value);
    }
}