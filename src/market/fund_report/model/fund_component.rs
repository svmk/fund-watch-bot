use crate::market::common::model::ticker::Ticker;
use crate::market::common::model::share::Share;
use crate::market::common::model::original_price::OriginalPrice;
use crate::market::fund_report::model::weight::Weight;
use crate::market::fund_report::model::fund_component_share::FundComponentShare;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundComponent {
    #[serde(rename = "ticker")]
    ticker: Ticker,
    #[serde(rename = "fund_component_share")]
    fund_component_share: FundComponentShare,
}

impl FundComponent {
    pub fn new(
        ticker: Ticker,
        share: Share,
        price: OriginalPrice,
        weight: Weight,
    ) -> FundComponent {
        let fund_component_share = FundComponentShare::new(
            share,
            price,
            weight,
        );
        return FundComponent {
            ticker,
            fund_component_share,
        }; 
    }

    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_share(&self) -> &FundComponentShare {
        return &self.fund_component_share;
    }
}