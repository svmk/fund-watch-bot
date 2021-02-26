use crate::prelude::*;
use crate::telegram::service::router::Router;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_messages::ChatMessages;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::message_id::MessageId;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::Service;
use tbot::contexts::Text as TextContext;
use tbot::types::parameters::Text as MessageText;
use tbot::types::message::Id as TelegramMessageId;
use std::str::FromStr;

#[derive(new)]
pub struct MessageHandler {
    router: Service<Router>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
    messages_repository: Service<RepositoryInstance<ChatId, ChatMessages>>,
}

impl MessageHandler {
    pub async fn handle_text_message(&self, context: &TextContext) -> Result<(), Failure> {
        self.ensure_chat_exists(context).await?;
        let incoming_message = IncomingMessage::from_str(&context.text.value)?;
        let message_handler = self.router.get_command(incoming_message.get_command())?;
        let chat_id = ChatId::from_i64(context.chat.id.0)?;
        let chat_context = ChatContext {
            chat_id,
        };
        let view = message_handler
            .handle_message(&chat_context, incoming_message).await?;
        let bot = &context.bot;
        let chat_messages = self
            .messages_repository
            .find(&chat_context.chat_id).await?;
        let mut chat_messages = match chat_messages {
            Some(chat_messages) => chat_messages,
            None => {
                ChatMessages::new(chat_context.chat_id.clone())
            },
        };
        for message in view.iter_messages() {
            let message_text = MessageText::with_plain(message.get_text());
            match chat_messages.get_telegram_message(message.get_id()) {
                Some(telegram_message_id) => {
                    let telegram_message_id = TelegramMessageId(telegram_message_id.to_u32());
                    let bot_message = bot.edit_message_text(
                        context.chat.id, 
                        telegram_message_id,
                        message_text,
                    );
                    bot_message.call().await?;
                },
                None => {
                    let bot_message = bot.send_message(context.chat.id, message_text);
                    let send_message_response = bot_message.call().await?;
                    let telegram_message_id = MessageId::from_u32(send_message_response.id.0)?;
                    chat_messages.assign_message(telegram_message_id, message.get_id().clone());
                },
            }
            self.messages_repository.store(&chat_messages).await?;
        }
        return Ok(());
    }

    async fn ensure_chat_exists(&self, context: &TextContext) -> Result<(), Failure> {
        let chat_id = ChatId::from_i64(context.chat.id.0)?;
        let is_chat_exists = self.chat_repository.find(&chat_id).await?.is_some();
        if !is_chat_exists {
            let chat = Chat::new(chat_id);
            self.chat_repository.store(&chat).await?;
        }
        return Ok(());
    }
}