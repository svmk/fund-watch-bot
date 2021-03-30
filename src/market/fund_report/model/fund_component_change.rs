use crate::market::common::model::ticker::Ticker;
use crate::market::fund_report::model::fund_component_sell::FundComponentSell;
use crate::market::fund_report::model::share_change::ShareChange;
use crate::market::fund_report::model::price_change::PriceChange;
use crate::market::fund_report::model::weight_change::WeightChange;
use crate::market::fund_report::model::fund_component_buy::FundComponentBuy;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct FundComponentChange {
    #[serde(rename = "ticker")]
    ticker: Ticker,
    #[serde(rename = "share_change")]
    share_change: ShareChange,
    #[serde(rename = "price_change")]
    price_change: PriceChange,
    #[serde(rename = "weight_change")]
    weight_change: WeightChange,
}

impl FundComponentChange {
    pub fn generate_fund_component_buy(&self) -> Option<FundComponentBuy> {
        let share = self.share_change.compute_buy();
        let price = self.price_change.get_to().clone();
        let weight = self.weight_change.compute_buy();
        if let Some((share, weight)) = share.zip(weight) {
            let result = FundComponentBuy::new(
                self.ticker.clone(),
                share,
                price,
                weight,
            );
            return Some(result);
        }
        return None;
    }

    pub fn generate_fund_component_sell(&self) -> Option<FundComponentSell> {
        let share = self.share_change.compute_sell();
        let price = self.price_change.get_to().clone();
        let weight = self.weight_change.compute_sell();
        if let Some((share, weight)) = share.zip(weight) {
            let result = FundComponentSell::new(
                self.ticker.clone(),
                share,
                price,
                weight,
            );
            return Some(result);
        }
        return None;
    }
}