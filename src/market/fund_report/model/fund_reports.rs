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

    pub fn generate_fund_change_ids(&self) -> impl Iterator<Item=FundChangesId> + '_ {
        let iterator = self
            .daily_reports
            .iter()
            .zip(
                self.daily_reports.iter().skip(1)
            )
            .map(|(prev_id, next_id)| {
                return FundChangesId::new(prev_id.clone(), next_id.clone());
            });
        return iterator;
    }

    pub fn push_once_daily_fund_report_id(&mut self, id: DailyFundReportId) {
        let _ = self.daily_reports.insert(id);
    }
}

impl Entity<FundId> for FundReports {
    fn get_entity_id(&self) -> &FundId {
        return &self.fund_id;
    }   
}