use crate::market::common::model::ticker::Ticker;
use crate::market::common::model::share::Share;
use crate::market::common::model::price::Price;
use crate::market::fund_report::model::weight::Weight;
use crate::market::fund_report::model::fund_component_share::FundComponentShare;

#[derive(Debug, Clone)]
pub struct FundComponent {
    ticker: Ticker,
    fund_component_share: FundComponentShare,
}

impl FundComponent {
    pub fn new(
        ticker: Ticker,
        share: Share,
        price: Price,
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