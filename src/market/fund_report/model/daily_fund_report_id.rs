use crate::repository::model::identity::Identity;
use crate::market::fund_report::model::fund_id::FundId;
use crate::app::model::date::Date;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DailyFundReportId {
    #[serde(rename = "fund_id")]
    fund_id: FundId,
    #[serde(rename = "date")]
    date: Date,
}

impl DailyFundReportId {
    pub fn get_fund_id(&self) -> &FundId {
        return &self.fund_id;
    }
}

impl Identity for DailyFundReportId {
    fn to_string(&self) -> String {
        return format!("{}_{}", self.date, self.fund_id);
    }
}