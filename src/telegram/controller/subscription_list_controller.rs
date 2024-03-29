use crate::telegram::controller::prelude::*;

use crate::telegram::model::chat::Chat;
use crate::repository::repository::repository_instance::RepositoryInstance;

use crate::market::fund_report::model::fund::Fund;
use crate::telegram::action::subscription_list_action::{SubscriptionListAction, SubscriptionListActionDecision};

use crate::telegram::controller::fund_info_controller::FundInfoController;
use crate::telegram::views::subscription_list_view::subscription_list_view;
use typed_di::service::service::Service;

#[derive(new)]
pub struct SubscriptionListController {
    chat_repository: Service<RepositoryInstance<Chat>>,
    fund_repository: Service<RepositoryInstance<Fund>>,
    action_repository: Service<RepositoryInstance<SubscriptionListAction>>,
    fund_info_controller: Service<FundInfoController>,
}

#[async_trait]
impl CommandHandler for SubscriptionListController {
    async fn handle_message(&self, context: &ChatContext, _message: IncomingMessage) -> Result<View, Failure> {
        let chat = self
            .chat_repository
            .get(&context.chat_id).await?;
        let funds = self
            .fund_repository
            .get_many(chat.get_fund_subscriptions()).await?;
        let fund_list_action  = SubscriptionListAction::new_fund_list(&funds, chat.get_fund_subscriptions());
        self.action_repository.store(&fund_list_action).await?;
        let view = subscription_list_view(&fund_list_action);
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for SubscriptionListController {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let mut chat = self
            .chat_repository
            .get(&context.chat_id).await?;
        let mut action = self
            .action_repository
            .get(action_route.get_action_id()).await?;
        action.update_subscriptions(chat.get_fund_subscriptions());
        match action.decide(&action_route) {
            SubscriptionListActionDecision::View(fund_id) => {
                let view = self.fund_info_controller.render(&chat, &fund_id).await?;
                return Ok(view);
            },
            SubscriptionListActionDecision::Unsubscribe(fund_id) => {
                chat.unsubscribe(&fund_id);
                action.update_subscriptions(chat.get_fund_subscriptions());
            },
            SubscriptionListActionDecision::SelectPage(page) => {
                action.select_page(&page)?;
            },
            SubscriptionListActionDecision::UnknownRoute => {
                return crate::fail!("Unknown action route {}", action_route);
            },
        }
        self.action_repository.store(&action).await?;
        self.chat_repository.store(&chat).await?;
        let view = subscription_list_view(&action);
        return Ok(view);
    }
}