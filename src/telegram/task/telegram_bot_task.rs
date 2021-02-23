use crate::prelude::*;
use crate::telegram::service::router::Router;
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
    router: Service<Router>,
}

impl TelegramBotTask {
    pub async fn run(&self) -> Result<(), Failure> {
        let bot = Bot::new(self.config.get_token().to_string());
        let mut event_loop = bot.event_loop();
        event_loop.text(|context| async move {
            
        });    
        event_loop
            .polling()
            .start()
            .await
            .map_err(|error|{
                return Failure::msg(format!("{:?}", error));
            })?;
        unimplemented!()
    }
}