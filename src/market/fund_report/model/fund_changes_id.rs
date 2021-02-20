use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::repository::model::identity::Identity;

#[derive(new, Debug)]
pub struct FundChangesId {
    prev_fund_id: DailyFundReportId,
    next_fund_id: DailyFundReportId,
}

impl Identity for FundChangesId {
    fn to_string(&self) -> String {
        return format!("{}-{}", self.prev_fund_id.to_string(), self.next_fund_id.to_string());
    }
}