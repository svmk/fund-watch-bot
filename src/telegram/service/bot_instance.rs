use crate::prelude::*;
use crate::telegram::model::view::View;
use crate::telegram::model::message_id::MessageId;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat_messages::ChatMessages;
use crate::telegram::utils::telegram_create_reply_markup::telegram_create_reply_markup;
use crate::repository::repository::repository_instance::RepositoryInstance;
use typed_di::service::Service;
use tbot::Bot;
use tbot::types::parameters::Text as MessageText;
use tbot::types::message::Id as TelegramMessageId;
use tbot::types::chat::Id as TelegramChatId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotInstanceConfig {
    #[serde(rename="token")]
    token: String,
}

impl BotInstanceConfig {
    pub fn get_token(&self) -> &String {
        return &self.token;
    }
}


pub struct BotInstance {
    config: BotInstanceConfig,
    messages_repository: Service<RepositoryInstance<ChatId, ChatMessages>>,
    bot: Bot,
}

impl BotInstance {
    pub fn new(
        config: BotInstanceConfig,
        messages_repository: Service<RepositoryInstance<ChatId, ChatMessages>>,
    ) -> BotInstance {
        let bot = Bot::new(config.get_token().to_string());
        return BotInstance {
            config,
            messages_repository,
            bot,
        }
    }

    pub fn get_bot(&self) -> Bot {
        return self.bot.clone();
    }

    pub async fn send_view(&self, chat_id: ChatId, view: View) -> Result<(), Failure> {
        let chat_messages = self
            .messages_repository
            .find(&chat_id).await?;
        let mut chat_messages = match chat_messages {
            Some(chat_messages) => chat_messages,
            None => {
                ChatMessages::new(chat_id.clone())
            },
        };
        let telegram_chat_id = TelegramChatId(chat_id.to_i64());
        for message in view.iter_messages() {
            let message_text = MessageText::with_plain(message.get_text());
            match chat_messages.get_telegram_message(message.get_id()) {
                Some(telegram_message_id) => {
                    let telegram_message_id = TelegramMessageId(telegram_message_id.to_u32());
                    let mut bot_message = self.bot.edit_message_text(
                        telegram_chat_id, 
                        telegram_message_id,
                        message_text,
                    );
                    if let Some(reply_markup) = telegram_create_reply_markup(message.get_reply_markup()) {
                        bot_message = bot_message.reply_markup(reply_markup);
                    }
                    bot_message.call().await?;
                },
                None => {
                    let mut bot_message = self.bot.send_message(telegram_chat_id, message_text);
                    if let Some(reply_markup) = telegram_create_reply_markup(message.get_reply_markup()) {
                        bot_message = bot_message.reply_markup(reply_markup);
                    }
                    let send_message_response = bot_message.call().await?;
                    let telegram_message_id = MessageId::from_u32(send_message_response.id.0)?;
                    chat_messages.assign_message(telegram_message_id, message.get_id().clone());
                },
            }
            self.messages_repository.store(&chat_messages).await?;
        }
        return Ok(());
    }
}