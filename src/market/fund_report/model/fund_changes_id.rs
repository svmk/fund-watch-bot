use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::repository::model::identity::Identity;

#[derive(new, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FundChangesId {
    #[serde(rename = "prev_fund_id")]
    prev_fund_id: DailyFundReportId,
    #[serde(rename = "next_fund_id")]
    next_fund_id: DailyFundReportId,
}

impl FundChangesId {
    pub fn get_prev_fund_id(&self) -> &DailyFundReportId {
        return &self.prev_fund_id;
    }
    
    pub fn get_next_fund_id(&self) -> &DailyFundReportId {
        return &self.next_fund_id;
    }
}

impl Identity for FundChangesId {
    fn to_string(&self) -> String {
        return format!("{}-{}", self.prev_fund_id.to_string(), self.next_fund_id.to_string());
    }
}