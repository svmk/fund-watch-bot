use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::repository::model::entity::Entity;
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct FundReports {
    #[serde(rename = "fund_id")]
    fund_id: FundId,
    #[serde(rename = "daily_reports")]
    daily_reports: BTreeSet<DailyFundReportId>,
    #[serde(rename = "fund_changes_ids")]
    fund_changes_ids: BTreeSet<FundChangesId>,
}

impl FundReports {
    pub fn new(fund_id: FundId) -> FundReports {
        return FundReports {
            fund_id,
            daily_reports: BTreeSet::new(),
            fund_changes_ids: BTreeSet::new(),
        };
    }
}

impl Entity<FundId> for FundReports {
    fn get_entity_id(&self) -> &FundId {
        return &self.fund_id;
    }   
}