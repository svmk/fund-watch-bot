use crate::market::common::model::ticker::Ticker;
use crate::app::model::year_quartal::YearQuartal;
use crate::repository::model::identity::Identity;

#[derive(Debug)]
pub struct QuartalPriceId {
    ticker: Ticker,
    period: YearQuartal,
}

impl Identity for QuartalPriceId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.ticker, self.period);
    }
}