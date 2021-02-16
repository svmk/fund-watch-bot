use crate::market::common::model::share::Share;
use crate::market::common::model::price::Price;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone)]
pub struct FundComponentShare {
    share: Share,
    price: Price,
    weight: Weight,
}

impl FundComponentShare {
    pub fn get_share(&self) -> &Share {
        return &self.share;
    }

    pub fn get_price(&self) -> &Price {
        return &self.price;
    }

    pub fn get_weight(&self) -> &Weight {
        return &self.weight;
    }   
}