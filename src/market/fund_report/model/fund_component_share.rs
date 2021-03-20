use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentShare {
    #[serde(rename = "share")]
    share: Share,
    #[serde(rename = "price")]
    price: OriginalPrice,
    #[serde(rename = "weight")]
    weight: Weight,
}

impl FundComponentShare {
    pub fn get_share(&self) -> &Share {
        return &self.share;
    }

    pub fn get_price(&self) -> &OriginalPrice {
        return &self.price;
    }

    pub fn get_weight(&self) -> &Weight {
        return &self.weight;
    }   
}