use crate::prelude::*;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::fund_report::service::fund_changes_generator::FundChangesGenerator;
use crate::market::fund_report::events::new_daily_fund_report_event::NewDailyFundReportEvent;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::fund_reports::FundReports;
use crate::market::fund_report::model::fund_id::FundId;
use crate::event_emitter::service::event_emitter::EventEmitter;
use crate::event_emitter::prelude::*;
use typed_di::service::Service;

#[derive(new)]
pub struct FundReportsEventListener {
    event_emitter: Service<EventEmitter>,
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
        return Ok(());
    }
}

impl EventListener<NewDailyFundReportEvent> for FundReportsEventListener {
    fn handle_event(&self, event: EventRecord<NewDailyFundReportEvent>) -> BoxFuture<Result<(), Failure>> {
        return self.handle_new_daily_fund_report_event(event).boxed();
    }
}
