use crate::market::fund_report::model::fund_reports::FundReports;

use crate::telegram::{controller::prelude::*};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::views::fund_change_list_view::fund_change_list_view;
use crate::telegram::service_handlers::action_handler::ActionHandler;

use crate::telegram::controller::fund_change_info_controller::FundChangeInfoController;
use crate::telegram::action::fund_change_list_action::{FundChangeListAction, FundChangeListActionDecision};
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundChangeListController {
    fund_repository: Service<RepositoryInstance<Fund>>,
    fund_reports_repository: Service<RepositoryInstance<FundReports>>,
    action_repository: Service<RepositoryInstance<FundChangeListAction>>,
    fund_change_info_controller: Service<FundChangeInfoController>,
}

impl FundChangeListController {
    pub async fn render(&self, fund_id: &FundId) -> Result<View, Failure> {
        let fund = self.fund_repository.get(fund_id).await?;
        let fund_reports = self.fund_reports_repository.get(fund_id).await?;
        let action = FundChangeListAction::new(&fund, fund_reports.get_fund_changes());
        self.action_repository.store(&action).await?;
        let view = fund_change_list_view(&action);
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for FundChangeListController {
    async fn handle_action(&self, _context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let mut action = self
            .action_repository
            .get(action_route.get_action_id()).await?;
        let action_decision = action.decide(&action_route);
        match action_decision {
            FundChangeListActionDecision::View(fund_changes_id) => {
                let view = self
                    .fund_change_info_controller
                    .render(&fund_changes_id).await?;
                return Ok(view);
            },
            FundChangeListActionDecision::SelectPage(page) => {
                action.select_page(&page)?;
            },
            FundChangeListActionDecision::Unknown => {
                return crate::fail!("Unknown action route {}", action_route);
            },
        }
        self.action_repository.store(&action).await?;
        let view = fund_change_list_view(&action);
        return Ok(view);
    }
}