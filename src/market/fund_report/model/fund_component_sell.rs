use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::common::model::ticker::Ticker;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentSell {
    #[serde(rename = "ticker")]
    ticker: Ticker,
    #[serde(rename = "share")]
    sold_share: Share,
    #[serde(rename = "sell_price")]
    sell_price: OriginalPrice,
    #[serde(rename = "weight")]
    sold_weight: Weight,
}

impl FundComponentSell {
    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_sold_share(&self) -> &Share {
        return &self.sold_share;
    }

    pub fn get_sell_price(&self) -> &OriginalPrice {
        return &self.sell_price;
    }

    pub fn get_sold_weight(&self) -> &Weight {
        return &self.sold_weight;
    }   
}