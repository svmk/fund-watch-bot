use crate::prelude::*;
use crate::telegram::service::command_router::CommandRouter;
use crate::telegram::service::action_router::ActionRouter;
use crate::telegram::service::bot_instance::BotInstance;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::action_route::ActionRoute;
use tbot::types::message::Message;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::service::Service;
use tbot::types::callback::query::Kind as QueryKind;
use tbot::types::chat::Id as TelegramChatId;
use tbot::types::message::Kind as MessageKind;
use tbot::types::callback::query::Query;
use std::str::FromStr;

#[derive(new)]
pub struct MessageHandler {
    command_router: Service<CommandRouter>,
    action_router: Service<ActionRouter>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
    bot_instance: Service<BotInstance>,
}

impl MessageHandler {
    pub async fn handle_text_message(&self, message: &Message) -> Result<(), Failure> {
        self.ensure_chat_exists(message.chat.id.clone()).await?;
        let text = match message.kind {
            MessageKind::Text(ref text) => text,
            _ => {
                return Ok(());
            },
        };
        let incoming_message = IncomingMessage::from_str(&text.value)?;
        let message_handler = self.command_router.get_command(incoming_message.get_command())?;
        let chat_id = ChatId::from_i64(message.chat.id.0)?;
        let chat_context = ChatContext {
            chat_id,
        };
        let view = message_handler
            .handle_message(&chat_context, incoming_message).await?;
        self.bot_instance.send_view(chat_context.chat_id.clone(), view).await?;
        return Ok(());
    }

    pub async fn handle_callback_message(&self, query: &Query) -> Result<(), Failure> {
        let chat_id = TelegramChatId(query.from.id.0);
        self.ensure_chat_exists(chat_id).await?;
        let action_route = match query.kind {
            QueryKind::Data(ref action_route) => action_route,
            _ => {
                return crate::fail!("Query kind cannot be game");
            },
        };
        let action_route = ActionRoute::from_str(&&action_route)?;
        let action_type = action_route.get_action_id().get_action_type();
        let action_handler = self.action_router.get_action_handler(action_type)?;
        let chat_id = ChatId::from_i64(chat_id.0)?;
        let chat_context = ChatContext {
            chat_id,
        };
        let view = action_handler.handle_action(&chat_context, action_route).await?;
        self.bot_instance.send_view(chat_context.chat_id.clone(), view).await?;
        return Ok(());
    }

    async fn ensure_chat_exists(&self, chat_id: TelegramChatId) -> Result<(), Failure> {
        let chat_id = ChatId::from_i64(chat_id.0)?;
        let is_chat_exists = self.chat_repository.find(&chat_id).await?.is_some();
        if !is_chat_exists {
            let chat = Chat::new(chat_id);
            self.chat_repository.store(&chat).await?;
        }
        return Ok(());
    }
}