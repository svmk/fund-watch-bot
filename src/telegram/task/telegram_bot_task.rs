use crate::{prelude::*};
use crate::telegram::service::message_handler::MessageHandler;
use crate::telegram::service::bot_instance::BotInstance;
use crate::telegram::views::bot_command_settings_view::bot_command_settings_view;
use typed_di::service::service::Service;
use tbot::types::update::Kind;

#[derive(new)]
pub struct TelegramBotTask {
    message_handler: Service<MessageHandler>,
    bot_instance: Service<BotInstance>,
}

impl TelegramBotTask {
    pub async fn run(&self) -> Result<(), Failure> {
        let bot = self.bot_instance.get_bot();
        self.register_bot_command_settings().await?;
        let mut event_loop = bot.event_loop();
        let message_handler = self.message_handler.clone();
        event_loop.unhandled(move |context| {
            let message_handler = message_handler.clone();
            return async move {
                let result = match context.update {
                    Kind::Message(ref message) => {
                        message_handler.handle_text_message(&message).await
                    },
                    Kind::CallbackQuery(ref query) => {
                        message_handler.handle_callback_message(&query).await
                    },
                    _ => {
                        Ok(())
                    },
                };
                if let Err(error) = result {
                    eprintln!("Telegram error: {}", error);
                }
            }
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

    async fn register_bot_command_settings(&self) -> Result<(), Failure> {
        let settings = bot_command_settings_view();
        self.bot_instance.register_bot_command_settings(settings).await?;
        return Ok(());
    }
}