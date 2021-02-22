use crate::prelude::*;
use crate::market::fund_report::model::fund_id::FundId;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use typed_di::service::Service;

pub struct FundChangesGenerator {
    report_repository: Service<RepositoryInstance<DailyFundReportId, DailyFundReport>>,
    fund_changes_repository: Service<RepositoryInstance<FundChangesId, FundChanges>>,
}

impl FundChangesGenerator {
    pub async fn generate_fund_changes(&self, fund_changes_id: FundChangesId) -> Result<FundChanges, Failure> {
        let fund_changes = self.fund_changes_repository.find(&fund_changes_id).await?;
        let fund_changes = match fund_changes {
            Some(fund_changes) => fund_changes,
            None => {
                let prev_report = self.report_repository.get(fund_changes_id.get_prev_fund_id()).await?;
                let next_report = self.report_repository.get(fund_changes_id.get_next_fund_id()).await?;
                let fund_changes = FundChanges::generate(&prev_report, &next_report)?;
                self.fund_changes_repository.store(&fund_changes).await?;
                fund_changes
            },
        };
        return Ok(fund_changes);
    }   
}