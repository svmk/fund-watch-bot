use crate::prelude::*;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::fund_report::service::fund_changes_generator::FundChangesGenerator;
use crate::market::fund_report::events::new_daily_fund_report_event::NewDailyFundReportEvent;
use crate::market::fund_report::model::fund_reports::FundReports;
use crate::market::fund_report::model::fund_id::FundId;
use crate::event_emitter::prelude::*;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundReportsEventListener {
    fund_changes_generator: Service<FundChangesGenerator>,
    fund_reports_repository: Service<RepositoryInstance<FundId, FundReports>>,
}

impl FundReportsEventListener {
    pub async fn handle_new_daily_fund_report_event(self: Service<Self>, event: EventRecord<NewDailyFundReportEvent>) -> Result<(), Failure> {
        let daily_fund_report_id = event
            .get_payload()
            .get_daily_fund_report_id()
            .clone();
        let fund_id = daily_fund_report_id.get_fund_id();
        let fund_reports = self
            .fund_reports_repository
            .find(fund_id).await?;
        let mut fund_reports = match fund_reports {
            Some(fund_reports) => fund_reports,
            None => {
                FundReports::new(fund_id.clone())
            }
        };
        fund_reports.push_once_daily_fund_report_id(daily_fund_report_id);
        self.generate_fund_changes(&mut fund_reports).await?;
        self.fund_reports_repository.store(&fund_reports).await?;
        return Ok(());
    }

    async fn generate_fund_changes(&self, fund_reports: &mut FundReports) -> Result<(), Failure> {
        let mut fund_change_ids = Vec::new();
        for fund_change_id in fund_reports.generate_fund_change_ids() {
            let fund_change = self.fund_changes_generator.generate_fund_changes(fund_change_id).await?;
            fund_change_ids.push(fund_change.get_id().clone());
        }
        for fund_change_id in fund_change_ids.into_iter() {
            fund_reports.push_once_daily_fund_change_id(fund_change_id);
        }
        return Ok(());
    }
}
