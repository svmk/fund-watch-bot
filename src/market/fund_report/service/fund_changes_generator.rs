use crate::prelude::*;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;

use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::market::fund_report::events::new_fund_change_event::NewFundChangeEvent;
use crate::event_emitter::service::event_emitter::EventEmitter;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundChangesGenerator {
    report_repository: Service<RepositoryInstance<DailyFundReport>>,
    fund_changes_repository: Service<RepositoryInstance<FundChanges>>,
    event_emitter: Service<EventEmitter>,
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
                self.fund_changes_repository.store(&fund_changes).await?;
                self.event_emitter.emit_event(NewFundChangeEvent::new(fund_changes.get_id().clone())).await?;
                fund_changes
            },
        };
        return Ok(fund_changes);
    }   
}