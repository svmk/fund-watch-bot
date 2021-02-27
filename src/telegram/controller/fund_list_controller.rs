use crate::telegram::controller::prelude::*;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat::Chat;
use crate::telegram::action::fund_list_action::{FundListAction, FundListActionDecision};
use crate::telegram::views::fund_list_view::fund_list_view;
use crate::telegram::model::action_id::ActionId;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::query::all_query::AllQuery;
use typed_di::service::Service;

#[derive(new)]
pub struct FundListController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
    action_repository: Service<RepositoryInstance<ActionId, FundListAction>>,
}


#[async_trait]
impl CommandHandler for FundListController {
    async fn handle_message(&self, context: &ChatContext, message: IncomingMessage) -> Result<View, Failure> {
        let mut funds = self
            .fund_repository
            .query(AllQuery::new()).await?
            .to_vec().await?;
        if let Some(argument) = message.get_argument() {
            let argument = argument.to_lowercase();
            funds.drain_filter(|fund| {
                let company_name = fund.get_company_name().as_str().to_lowercase();
                return !company_name.contains(&argument);
            }).for_each(|_|{});
        }
        funds.sort_by_key(|fund| {
            return fund.get_company_name().as_str().to_lowercase();
        });
        let chat = self.chat_repository.get(&context.chat_id).await?;
        let fund_list_action  = FundListAction::new(&funds, chat.get_fund_subscriptions());
        self.action_repository.store(&fund_list_action).await?;
        let view = fund_list_view(&fund_list_action);
        return Ok(view);
    }
}

#[async_trait]
impl ActionHandler for FundListController {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        let mut chat = self
            .chat_repository
            .get(&context.chat_id).await?;
        let mut action = self
            .action_repository
            .get(action_route.get_action_id()).await?;
        action.update_subscriptions(chat.get_fund_subscriptions());
        match action.decide(&action_route) {
            FundListActionDecision::Subscribe(fund_id) => {
                chat.subscribe(fund_id);
            },
            FundListActionDecision::UnSubscribe(fund_id) => {
                chat.unsubscribe(&fund_id);
            },
            FundListActionDecision::Render => {},
        }
        self.action_repository.store(&action).await?;
        self.chat_repository.store(&chat).await?;
        let view = fund_list_view(&action);
        return Ok(view);
    }
}