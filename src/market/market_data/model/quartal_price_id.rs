use crate::market::common::model::ticker::Ticker;
use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::datetime::DateTime;
use crate::repository::model::identity::Identity;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct QuartalPriceId {
    #[serde(rename="ticker")]
    ticker: Ticker,
    #[serde(rename="quartal")]
    period: YearQuartal,
}

impl QuartalPriceId {
    pub fn from_ticker_and_date(ticker: Ticker, datetime: DateTime) -> QuartalPriceId {
        let period = YearQuartal::from_date(datetime.to_date());
        return QuartalPriceId {
            ticker,
            period,
        };
    }

    pub fn from_ticker_and_year_quartal(ticker: Ticker, period: YearQuartal) -> QuartalPriceId {
        return QuartalPriceId {
            ticker,
            period,
        };
    }

    pub fn get_ticker(&self) -> &Ticker {
        return &self.ticker;
    }

    pub fn get_period(&self) -> &YearQuartal {
        return &self.period;
    }
}

impl Identity for QuartalPriceId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.ticker, self.period);
    }
}