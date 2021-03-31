use crate::market::common::model::company_id::CompanyId;
use crate::app::model::year_quartal::YearQuartal;
use crate::app::model::datetime::DateTime;
use crate::repository::model::identity::Identity;
use std::fmt;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct QuartalPriceId {
    #[serde(rename="company_id")]
    company_id: CompanyId,
    #[serde(rename="quartal")]
    period: YearQuartal,
}

impl QuartalPriceId {
    pub fn from_ticker_and_date(company_id: CompanyId, datetime: DateTime) -> QuartalPriceId {
        let period = YearQuartal::from_date(datetime.to_date());
        return QuartalPriceId {
            company_id,
            period,
        };
    }

    pub fn from_ticker_and_year_quartal(company_id: CompanyId, period: YearQuartal) -> QuartalPriceId {
        return QuartalPriceId {
            company_id,
            period,
        };
    }

    pub fn get_company_id(&self) -> &CompanyId {
        return &self.company_id;
    }

    pub fn get_period(&self) -> &YearQuartal {
        return &self.period;
    }
}

impl Identity for QuartalPriceId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.company_id, self.period);
    }
}

impl fmt::Display for QuartalPriceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.company_id, self.period)
    }
}