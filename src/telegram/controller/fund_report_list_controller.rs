use crate::market::fund_report::model::fund_reports::FundReports;
use crate::telegram::{controller::prelude::*, model::action_id::ActionId};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::views::fund_report_list_view::fund_report_list_view;
use crate::telegram::service_handlers::action_handler::ActionHandler;
use crate::telegram::controller::fund_report_info_controller::FundReportInfoController;
use crate::telegram::action::fund_report_list_action::{FundReportListAction, FundReportListActionDecision};
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundReportListController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    fund_reports_repository: Service<RepositoryInstance<FundId, FundReports>>,
    action_repository: Service<RepositoryInstance<ActionId, FundReportListAction>>,
    fund_report_controller: Service<FundReportInfoController>,
}

impl FundReportListController {
    pub async fn render(&self, fund_id: &FundId) -> Result<View, Failure> {
        let fund = self.fund_repository.get(fund_id).await?;
        let fund_reports = self.fund_reports_repository.get(fund_id).await?;
        let action = FundReportListAction::new(&fund, fund_reports.get_daily_reports());
        self.action_repository.store(&action).await?;
        let view = fund_report_list_view(&action);
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for FundReportListController {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let mut action = self
            .action_repository
            .get(action_route.get_action_id()).await?;
        let action_decision = action.decide(&action_route);
        match action_decision {
            FundReportListActionDecision::View(daily_fund_report_id) => {
                let view = self
                    .fund_report_controller
                    .render(&daily_fund_report_id).await?;
                return Ok(view);
            },
            FundReportListActionDecision::SelectPage(page) => {
                action.select_page(&page)?;
            },
            FundReportListActionDecision::Unknown => {
                return crate::fail!("Unknown action route {}", action_route);
            },
        }
        self.action_repository.store(&action).await?;
        let view = fund_report_list_view(&action);
        return Ok(view);
    }
}