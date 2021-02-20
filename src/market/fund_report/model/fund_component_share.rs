use crate::market::common::model::share::Share;
use crate::market::common::model::historical_price::HistoricalPrice;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentShare {
    #[serde(rename = "share")]
    share: Share,
    #[serde(rename = "price")]
    price: HistoricalPrice,
    #[serde(rename = "weight")]
    weight: Weight,
}

impl FundComponentShare {
    pub fn get_share(&self) -> &Share {
        return &self.share;
    }

    pub fn get_price(&self) -> &HistoricalPrice {
        return &self.price;
    }

    pub fn get_weight(&self) -> &Weight {
        return &self.weight;
    }   
}