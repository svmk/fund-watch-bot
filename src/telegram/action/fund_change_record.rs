use crate::prelude::*;
use crate::market::common::model::company_id::CompanyId;
use crate::market::common::model::share::Share;
use crate::market::common::model::actual_price::ActualPrice;
use crate::market::fund_report::model::weight::Weight;

use crate::market::fund_report::model::fund_component_sell::FundComponentSell;
use crate::market::fund_report::model::fund_component_buy::FundComponentBuy;
use crate::market::market_data::model::split_rules::SplitRules;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundChangeRecord {
    #[serde(rename = "company_id")]
    company_id: CompanyId,
    #[serde(rename = "share")]
    share: Share,
    #[serde(rename = "price")]
    price: ActualPrice,
    #[serde(rename = "weight")]
    weight: Weight,
}

impl FundChangeRecord {
    pub fn from_sell(sell: &FundComponentSell, split_rules: &SplitRules) -> Result<FundChangeRecord, Failure> {
        let price = split_rules.calculate_actual_price(sell.get_sell_price())?;
        let sell = FundChangeRecord::new(
            sell.get_company_id().clone(),
            sell.get_sold_share().clone(),
            price,
            sell.get_sold_weight().clone(),
        );
        return Ok(sell);
    }

    pub fn from_buy(buy: &FundComponentBuy, split_rules: &SplitRules) -> Result<FundChangeRecord, Failure> {
        let price = split_rules.calculate_actual_price(buy.get_buy_price())?;
        let buy = FundChangeRecord::new(
            buy.get_company_id().clone(),
            buy.get_buyed_share().clone(),
            price,
            buy.get_buyed_weight().clone(),
        );
        return Ok(buy);
    }

    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
    }

    pub fn get_share(&self) -> &Share {
        return &self.share;
    }

    pub fn get_price(&self) -> &ActualPrice {
        return &self.price;
    }

    pub fn get_weight(&self) -> &Weight {
        return &self.weight;
    }   
}