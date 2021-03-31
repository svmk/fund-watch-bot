use crate::market::common::model::company_id::CompanyId;
use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentBuy {
    #[serde(rename = "company_id")]
    company_id: CompanyId,
    #[serde(rename = "share")]
    buyed_share: Share,
    #[serde(rename = "buy_price")]
    buy_price: OriginalPrice,
    #[serde(rename = "weight")]
    buyed_weight: Weight,
}

impl FundComponentBuy {
    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
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