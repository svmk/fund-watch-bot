use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::common::model::ticker::Ticker;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentBuy {
    #[serde(rename = "ticker")]
    ticker: Ticker,
    #[serde(rename = "share")]
    buyed_share: Share,
    #[serde(rename = "buy_price")]
    buy_price: OriginalPrice,
    #[serde(rename = "weight")]
    buyed_weight: Weight,
}

impl FundComponentBuy {
    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_buyed_share(&self) -> &Share {
        return &self.buyed_share;
    }

    pub fn get_buy_price(&self) -> &OriginalPrice {
        return &self.buy_price;
    }

    pub fn get_buyed_weight(&self) -> &Weight {
        return &self.buyed_weight;
    }   
}