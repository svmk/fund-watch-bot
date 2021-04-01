use crate::telegram::controller::prelude::*;
use crate::telegram::controller::fund_info_controller::FundInfoController;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::action::fund_list_action::{FundListAction, FundListActionDecision};
use crate::telegram::model::action_id::ActionId;
use crate::telegram::views::fund_list_view::fund_list_view;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::service::Service;

#[derive(new)]
pub struct FundListController {
    fund_repository: Service<RepositoryInstance<Fund>>,
    chat_repository: Service<RepositoryInstance<Chat>>,
    action_repository: Service<RepositoryInstance<FundListAction>>,
    fund_info_controller: Service<FundInfoController>,
}


#[async_trait]
impl CommandHandler for FundListController {
    async fn handle_message(&self, context: &ChatContext, message: IncomingMessage) -> Result<View, Failure> {
        let mut funds = self
            .fund_repository
            .all().await?
            .to_vec().await?;
        if let Some(argument) = message.get_argument() {
            let argument = argument.to_lowercase();
            funds.drain_filter(|fund| {
                let company_name = fund.get_company_name().as_str().to_lowercase();
                return !company_name.contains(&argument);
            }).for_each(|_|{});
        }
        let chat = self.chat_repository.get(&context.chat_id).await?;
        let fund_list_action  = FundListAction::new_fund_list(&funds, chat.get_fund_subscriptions());
        self.action_repository.store(&fund_list_action).await?;
        let view = fund_list_view(&fund_list_action);
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for FundListController {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let chat = self
            .chat_repository
            .get(&context.chat_id).await?;
        let mut action = self
            .action_repository
            .get(action_route.get_action_id()).await?;
        action.update_subscriptions(chat.get_fund_subscriptions());
        let action_decision = action.decide(&action_route);
        match action_decision {
            FundListActionDecision::View(fund_id) => {
                return self.fund_info_controller.render(&chat, &fund_id).await;
            },
            FundListActionDecision::SelectPage(page) => {
                action.select_page(&page)?;
            },
            FundListActionDecision::UnknownRoute => {
                return crate::fail!("Unknown action route {}", action_route);
            },
        }
        self.action_repository.store(&action).await?;
        self.chat_repository.store(&chat).await?;
        let view = fund_list_view(&action);
        return Ok(view);
    }
}