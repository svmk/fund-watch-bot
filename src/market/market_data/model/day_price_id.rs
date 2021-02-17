use crate::market::common::model::ticker::Ticker;
use crate::app::model::date::Date;
use crate::repository::model::identity::Identity;


#[derive(Debug, Serialize, Deserialize)]
pub struct DayPriceId {
    ticker: Ticker,
    date: Date,
}

impl Identity for DayPriceId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.ticker, self.date);
    }
}