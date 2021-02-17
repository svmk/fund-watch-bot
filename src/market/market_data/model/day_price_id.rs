use crate::market::common::model::ticker::Ticker;
use crate::app::model::date::Date;
use crate::repository::model::identity::Identity;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DayPriceId {
    ticker: Ticker,
    date: Date,
}

impl DayPriceId {
    pub fn from_ticker_and_date(ticker: Ticker, date: Date) -> DayPriceId {
        return DayPriceId {
            ticker,
            date,
        }
    }
}

impl Identity for DayPriceId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.ticker, self.date);
    }
}