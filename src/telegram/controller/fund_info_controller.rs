use crate::telegram::{controller::prelude::*, model::action_id::ActionId};
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_id::ChatId;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::views::fund_info_view::fund_info_view;
use crate::telegram::action::fund_info_action::{FundInfoAction, FundInfoActionDecision};
use crate::telegram::service_handlers::action_handler::ActionHandler;
use crate::telegram::controller::fund_report_list_controller::FundReportListController;
use crate::telegram::controller::fund_change_list_controller::FundChangeListController;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundInfoController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
    action_repository: Service<RepositoryInstance<ActionId, FundInfoAction>>,
    fund_report_list_controller: Service<FundReportListController>,
    fund_change_list_controller: Service<FundChangeListController>,
}

impl FundInfoController {
    pub async fn render(&self, chat: &Chat, fund_id: &FundId) -> Result<View, Failure> {
        let fund = self.fund_repository.get(fund_id).await?;
        let action = FundInfoAction::new(fund, chat.get_fund_subscriptions());
        let view = fund_info_view(&action);
        self.action_repository.store(&action).await?;
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for FundInfoController {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let mut chat = self.chat_repository.get(&context.chat_id).await?;
        let mut action = self.action_repository.get(action_route.get_action_id()).await?;
        let action_decision = action.decide(&action_route);
        match action_decision {
            FundInfoActionDecision::FundReportList => {
                let view = self
                    .fund_report_list_controller
                    .render(action.get_fund().get_fund_id()).await?;
                return Ok(view);
            },
            FundInfoActionDecision::FundChangeList => {
                let view = self
                    .fund_change_list_controller
                    .render(action.get_fund().get_fund_id()).await?;
                return Ok(view);
            },
            FundInfoActionDecision::Subscribe => {
                chat.subscribe(action.get_fund().get_fund_id().clone());
            },
            FundInfoActionDecision::Unsubscribe => {
                chat.unsubscribe(action.get_fund().get_fund_id());
            },
            FundInfoActionDecision::Unknown => {
                return crate::fail!("Unknown action route {}", action_route);
            },
        }
        action.update_subscription(chat.get_fund_subscriptions());
        self.action_repository.store(&action).await?;
        self.chat_repository.store(&chat).await?;
        let view = fund_info_view(&action);
        return Ok(view)
    }
}