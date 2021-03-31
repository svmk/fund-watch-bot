use crate::market::common::model::company_id::CompanyId;
use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentSell {
    #[serde(rename = "company_id")]
    company_id: CompanyId,
    #[serde(rename = "share")]
    sold_share: Share,
    #[serde(rename = "sell_price")]
    sell_price: OriginalPrice,
    #[serde(rename = "weight")]
    sold_weight: Weight,
}

impl FundComponentSell {
    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
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