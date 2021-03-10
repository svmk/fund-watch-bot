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
    async fn handle_new_daily_fund_report_event(&self, event: EventRecord<NewDailyFundReportEvent>) -> Result<(), Failure> {
        let daily_fund_report_id = event
            .get_payload()
            .get_daily_fund_report_id()
            .clone();
        let fund_id = daily_fund_report_id.get_fund_id();
        let mut fund_reports = self
            .fund_reports_repository
            .get(fund_id).await?;
        fund_reports.push_once_daily_fund_report_id(daily_fund_report_id);
        self.fund_reports_repository.store(&fund_reports).await?;
        self.generate_fund_changes(&fund_reports).await?;
        return Ok(());
    }

    async fn generate_fund_changes(&self, fund_reports: &FundReports) -> Result<(), Failure> {
        for fund_change_id in fund_reports.generate_fund_change_ids() {
            let _ = self.fund_changes_generator.generate_fund_changes(fund_change_id).await?;
        }
        return Ok(());
    }
}

#[async_trait]
impl EventHandler<NewDailyFundReportEvent> for FundReportsEventListener {
    async fn handle_event(&self, event: EventRecord<NewDailyFundReportEvent>) -> Result<(), Failure> {
        return self.handle_new_daily_fund_report_event(event).await;
    }
}
