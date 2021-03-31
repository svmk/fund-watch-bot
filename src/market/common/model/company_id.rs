use crate::market::common::model::ticker::Ticker;
use crate::market::common::model::cusip::Cusip;
use crate::repository::model::identity::Identity;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyId {
    #[serde(rename="cusip")]
    cusip: Cusip,
    #[serde(rename="ticker")]
    ticker: Option<Ticker>,
}

impl CompanyId {
    pub fn new(cusip: Cusip) -> CompanyId {
        return CompanyId {
            cusip,
            ticker: None,
        }
    }

    pub fn with_ticker(mut self, ticker: Ticker) -> CompanyId {
        self.ticker = Some(ticker);
        return self;
    }

    pub fn get_cusip(&self) -> &Cusip {
        return &self.cusip;
    }

    pub fn get_opt_ticker(&self) -> Option<&Ticker> {
        return self.ticker.as_ref();
    }
}

impl Identity for CompanyId {
    fn to_string(&self) -> String {
        return format!("{}", self.cusip);
    }
}

impl fmt::Display for CompanyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cusip)
    }
}

impl PartialEq for CompanyId {
    fn eq(&self, other: &CompanyId) -> bool {
        return self.cusip.eq(&other.cusip);
    }
}

impl Eq for CompanyId {

}

impl PartialOrd for CompanyId {
    fn partial_cmp(&self, other: &CompanyId) -> Option<Ordering> {
        return self.cusip.partial_cmp(&other.cusip);
    }
}

impl Ord for CompanyId {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.cusip.cmp(&other.cusip);
    }
}