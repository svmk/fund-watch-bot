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
    daily_reports: Vec<DailyFundReportId>,
    #[serde(rename = "fund_changes")]
    fund_changes: Vec<FundChangesId>,
}

impl FundReports {
    pub fn new(fund_id: FundId) -> FundReports {
        return FundReports {
            fund_id,
            daily_reports: Vec::new(),
            fund_changes: Vec::new(),
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
        if !self.daily_reports.contains(&id) {
            let _ = self.daily_reports.push(id);
            self.daily_reports.sort();
        }
    }

    pub fn get_daily_reports(&self) -> &Vec<DailyFundReportId> {
        return &self.daily_reports;
    }

    pub fn push_once_daily_fund_change_id(&mut self, id: FundChangesId) {
        if !self.fund_changes.contains(&id) {
            let _ = self.fund_changes.push(id);
            self.fund_changes.sort_by_key(|item| {
                return item.get_prev_fund_id().clone();
            });
        }
    }

    pub fn get_fund_changes(&self) -> &Vec<FundChangesId> {
        return &self.fund_changes;
    }
}

impl Entity<FundId> for FundReports {
    fn get_entity_id(&self) -> &FundId {
        return &self.fund_id;
    }   
}