use crate::market::model::fund_id::FundId;
use crate::app::model::date::Date;

#[derive(Debug, Clone, PartialEq)]
pub struct DailyFundReportId {
    fund_id: FundId,
    date: Date,
}

impl DailyFundReportId {
    pub fn new(fund_id: FundId) -> DailyFundReportId {
        return DailyFundReportId {
            fund_id,
            date: Date::today(),
        };
    }

    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }
}