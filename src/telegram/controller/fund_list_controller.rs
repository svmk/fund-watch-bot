use crate::telegram::controller::prelude::*;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat::Chat;
use crate::telegram::action::fund_list_action::FundListAction;
use crate::telegram::views::fund_list_view::fund_list_view;
use crate::market::fund_report::model::fund_id::FundId;
use crate::market::fund_report::model::fund::Fund;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::query::all_query::AllQuery;
use typed_di::service::Service;

#[derive(new)]
pub struct FundListController {
    fund_repository: Service<RepositoryInstance<FundId, Fund>>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
}


#[async_trait]
impl CommandHandler for FundListController {
    async fn handle_message(&self, context: &ChatContext, message: IncomingMessage) -> Result<View, Failure> {
        let funds = self
            .fund_repository
            .query(AllQuery::new()).await?
            .to_vec().await?;
        let chat = self.chat_repository.get(&context.chat_id).await?;
        let fund_list_action  = FundListAction::new(&funds, chat.get_fund_subscriptions());
        let view = fund_list_view(&fund_list_action);
        return Ok(view);
    }
}