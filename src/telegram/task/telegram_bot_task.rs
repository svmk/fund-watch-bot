use crate::prelude::*;
use crate::telegram::service::message_handler::MessageHandler;
use typed_di::service::Service;
use tbot::Bot;

#[derive(Debug)]
pub struct TelegramBotTaskConfig {
    token: String,
}

impl TelegramBotTaskConfig {
    pub fn get_token(&self) -> &String {
        return &self.token;
    }
}

#[derive(new)]
pub struct TelegramBotTask {
    config: TelegramBotTaskConfig,
    message_handler: Service<MessageHandler>,
}

impl TelegramBotTask {
    pub async fn run(&self) -> Result<(), Failure> {
        let bot = Bot::new(self.config.get_token().to_string());
        let mut event_loop = bot.event_loop();
        let message_handler = self.message_handler.clone();
        event_loop.text(move |context| {
            let message_handler = message_handler.clone();
            return async move {
                let result = message_handler.handle_text_message(&context).await;
                if let Err(error) = result {
                    eprintln!("Telegram error: {}", error);
                }
            };
        });
        let message_handler = self.message_handler.clone();
        event_loop.data_callback(move |context| {
            let message_handler = message_handler.clone();
            return async move {
                let result = message_handler.handle_callback_message(&context).await;
                if let Err(error) = result {
                    eprintln!("Telegram error: {}", error);
                }
            };
        });
        event_loop
            .polling()
            .start()
            .await
            .map_err(|error|{
                return Failure::msg(format!("{:?}", error));
            })?;
        return Ok(());
    }
}