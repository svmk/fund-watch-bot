use crate::telegram::{controller::prelude::*, model::action_id::ActionId};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::views::fund_change_info_view::fund_change_info_view;
use crate::telegram::action::fund_change_info_action::FundChangeInfoAction;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundChangeInfoController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    fund_changes_repository: Service<RepositoryInstance<FundChangesId, FundChanges>>,
    action_repository: Service<RepositoryInstance<ActionId, FundChangeInfoAction>>,
}

impl FundChangeInfoController {
    pub async fn render(&self, fund_changes_id: &FundChangesId) -> Result<View, Failure> {
        let fund_id = fund_changes_id.get_prev_fund_id().get_fund_id();
        let fund = self.fund_repository.get(fund_id).await?;
        let fund_change = self
            .fund_changes_repository
            .get(fund_changes_id).await?;
        let action = FundChangeInfoAction::new(
            &fund,
            &fund_change,
        );
        self.action_repository.store(&action).await?;
        let view = fund_change_info_view(&action);
        return Ok(view);
    }
}