use crate::prelude::*;
use crate::telegram::service::router::Router;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat::Chat;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::Service;
use tbot::contexts::Text as TextContext;
use tbot::types::parameters::Text as MessageText;
use std::str::FromStr;

#[derive(new)]
pub struct MessageHandler {
    router: Service<Router>,
    chat_repository: Service<RepositoryInstance<ChatId, Chat>>,
}

impl MessageHandler {
    pub async fn handle_text_message(&self, context: &TextContext) -> Result<(), Failure> {
        self.ensure_chat_exists(context).await?;
        let incoming_message = IncomingMessage::from_str(&context.text.value)?;
        let message_handler = self.router.get_command(incoming_message.get_command())?;
        let view = message_handler
            .handle_message(incoming_message).await?;
        let bot = &context.bot;
        for message in view.iter_messages() {
            let message_text = MessageText::with_plain(message.get_text());
            let bot_message = bot.send_message(context.chat.id, message_text);
            bot_message.call().await?;
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