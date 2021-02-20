use crate::market::common::model::ticker::Ticker;
use crate::market::fund_report::model::share_change::ShareChange;
use crate::market::fund_report::model::price_change::PriceChange;
use crate::market::fund_report::model::weight_change::WeightChange;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentChange {
    #[serde(rename = "ticker")]
    ticker: Ticker,
    #[serde(rename = "share_change")]
    share_change: Option<ShareChange>,
    #[serde(rename = "price_change")]
    price_change: Option<PriceChange>,
    #[serde(rename = "weight_change")]
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