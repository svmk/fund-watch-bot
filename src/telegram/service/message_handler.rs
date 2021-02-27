use crate::prelude::*;
use crate::telegram::service::command_router::CommandRouter;
use crate::telegram::service::action_router::ActionRouter;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::view::View;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_messages::ChatMessages;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::message_id::MessageId;
use crate::telegram::model::action_route::ActionRoute;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::Service;
use tbot::contexts::DataCallback;
use tbot::contexts::Text as TextContext;
use tbot::types::parameters::Text as MessageText;
use tbot::types::message::Id as TelegramMessageId;
use tbot::Bot;
use tbot::types::chat::Id as TelegramChatId;
use std::str::FromStr;

#[derive(new)]
pub struct MessageHandler {
    command_router: Service<CommandRouter>,
    action_router: Service<ActionRouter>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
    messages_repository: Service<RepositoryInstance<ChatId, ChatMessages>>,
}

impl MessageHandler {
    pub async fn handle_text_message(&self, context: &TextContext) -> Result<(), Failure> {
        self.ensure_chat_exists(context.chat.id.clone()).await?;
        let incoming_message = IncomingMessage::from_str(&context.text.value)?;
        let message_handler = self.command_router.get_command(incoming_message.get_command())?;
        let chat_id = ChatId::from_i64(context.chat.id.0)?;
        let chat_context = ChatContext {
            chat_id,
        };
        let bot = &context.bot;
        let view = message_handler
            .handle_message(&chat_context, incoming_message).await?;
        self.send_view(bot.as_ref(), &chat_context, view).await?;
        return Ok(());
    }

    pub async fn handle_callback_message(&self, context: &DataCallback) -> Result<(), Failure> {
        let chat_id = TelegramChatId(context.from.id.0);
        self.ensure_chat_exists(chat_id).await?;
        let action_route = ActionRoute::from_str(&context.data)?;
        let action_type = action_route.get_action_id().get_action_type();
        let action_handler = self.action_router.get_action_handler(action_type)?;
        let chat_id = ChatId::from_i64(chat_id.0)?;
        let chat_context = ChatContext {
            chat_id,
        };
        let bot = &context.bot;
        let view = action_handler.handle_action(&chat_context, action_route).await?;
        self.send_view(bot.as_ref(), &chat_context, view).await?;
        return Ok(());
    }

    async fn send_view(&self, bot: &Bot, chat_context: &ChatContext, view: View) -> Result<(), Failure> {
        let chat_messages = self
            .messages_repository
            .find(&chat_context.chat_id).await?;
        let mut chat_messages = match chat_messages {
            Some(chat_messages) => chat_messages,
            None => {
                ChatMessages::new(chat_context.chat_id.clone())
            },
        };
        let telegram_chat_id = TelegramChatId(chat_context.chat_id.to_i64());
        for message in view.iter_messages() {
            let message_text = MessageText::with_plain(message.get_text());
            match chat_messages.get_telegram_message(message.get_id()) {
                Some(telegram_message_id) => {
                    let telegram_message_id = TelegramMessageId(telegram_message_id.to_u32());
                    let bot_message = bot.edit_message_text(
                        telegram_chat_id, 
                        telegram_message_id,
                        message_text,
                    );
                    bot_message.call().await?;
                },
                None => {
                    let bot_message = bot.send_message(telegram_chat_id, message_text);
                    let send_message_response = bot_message.call().await?;
                    let telegram_message_id = MessageId::from_u32(send_message_response.id.0)?;
                    chat_messages.assign_message(telegram_message_id, message.get_id().clone());
                },
            }
            self.messages_repository.store(&chat_messages).await?;
        }
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