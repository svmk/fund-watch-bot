use crate::telegram::service::bot_instance::BotInstance;
use crate::market::fund_report::events::new_fund_change_event::NewFundChangeEvent;
use crate::market::fund_report::model::fund_changes::FundChanges;
use crate::market::fund_report::model::fund_changes_id::FundChangesId;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat::Chat;
use crate::telegram::query::chat_subscribed_to_fund_query::ChatSubscribedToFundQuery;
use crate::telegram::views::fund_change_view::fund_change_view;
use crate::event_emitter::prelude::*;
use crate::prelude::*;
use typed_di::service::Service;

#[derive(new)]
pub struct EventNotifier {
    bot_instance: Service<BotInstance>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
    fund_changes_repository: Service<RepositoryInstance<FundChangesId, FundChanges>>,
}

#[async_trait]
impl EventListener<NewFundChangeEvent> for EventNotifier {
    async fn handle_event(&self, event: EventRecord<NewFundChangeEvent>) -> Result<(), Failure> {
        let fund_id = event
            .get_payload()
            .get_fund_change_id()
            .get_prev_fund_id()
            .get_fund_id()
            .clone();
        let subscribed_chats = self
            .chat_repository
            .query(ChatSubscribedToFundQuery::new(fund_id)).await?
            .to_vec().await?;
        let fund_changes = self
            .fund_changes_repository
            .get(event.get_payload().get_fund_change_id()).await?;
        let view = fund_change_view(&fund_changes);
        for chat in subscribed_chats.iter() {
            
        }
        return Ok(());
    }
}
