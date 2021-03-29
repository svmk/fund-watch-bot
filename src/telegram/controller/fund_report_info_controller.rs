use crate::telegram::{controller::prelude::*, model::action_id::ActionId};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::market::fund_report::model::daily_fund_report_id::DailyFundReportId;
use crate::market::fund_report::model::daily_fund_report::DailyFundReport;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::views::fund_report_info_view::fund_report_info_view;
use crate::telegram::service_handlers::action_handler::ActionHandler;
use crate::telegram::action::fund_report_info_action::{FundReportInfoAction, FundReportInfoActionDecision};
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundReportInfoController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    daily_fund_reports_repository: Service<RepositoryInstance<DailyFundReportId, DailyFundReport>>,
    candlestick_provider: Service<CandlestickProvider>,
    action_repository: Service<RepositoryInstance<ActionId, FundReportInfoAction>>,
}

impl FundReportInfoController {
    pub async fn render(&self, daily_fund_report_id: &DailyFundReportId) -> Result<View, Failure> {
        let fund_id = daily_fund_report_id.get_fund_id();
        let fund = self.fund_repository.get(fund_id).await?;
        let fund_report = self
            .daily_fund_reports_repository
            .get(daily_fund_report_id).await?;
        let mut action = FundReportInfoAction::new_empty(&fund, &fund_report); 
        let fund_report_datetime = daily_fund_report_id.get_date().end_of_day();
        for component in fund_report.get_fund_components().iter() {
            let split_rules = self
                .candlestick_provider
                .fetch_split_rules(component.get_ticker(), &fund_report_datetime).await?;
            action.push_component(component, &split_rules)?;
        }
        self.action_repository.store(&action).await?;
        let view = fund_report_info_view(&action);
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for FundReportInfoController {
    async fn handle_action(&self, _context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let mut action = self
            .action_repository
            .get(action_route.get_action_id()).await?;
        let action_decision = action.decide(&action_route);
        match action_decision {
            FundReportInfoActionDecision::SelectPage(page) => {
                action.select_page(&page)?;
            },
            FundReportInfoActionDecision::Unknown => {
                return crate::fail!("Unknown action route {}", action_route);
            },
        }
        self.action_repository.store(&action).await?;
        let view = fund_report_info_view(&action);
        return Ok(view);
    }
}