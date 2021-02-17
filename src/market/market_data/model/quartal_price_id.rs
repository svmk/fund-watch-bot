use crate::market::common::model::ticker::Ticker;
use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::datetime::DateTime;
use crate::repository::model::identity::Identity;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuartalPriceId {
    ticker: Ticker,
    period: YearQuartal,
}

impl QuartalPriceId {
    pub fn from_ticker_and_date(ticker: Ticker, datetime: DateTime) -> QuartalPriceId {
        let period = YearQuartal::from_datetime(datetime);
        return QuartalPriceId {
            ticker,
            period,
        };
    }
}

impl Identity for QuartalPriceId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.ticker, self.period);
    }
}