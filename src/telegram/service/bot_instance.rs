use tbot::Bot;

#[derive(Debug)]
pub struct BotInstanceConfig {
    token: String,
}

impl BotInstanceConfig {
    pub fn get_token(&self) -> &String {
        return &self.token;
    }
}


pub struct BotInstance {
    config: BotInstanceConfig,
    bot: Bot,
}

impl BotInstance {
    pub fn new(config: BotInstanceConfig) -> BotInstance {
        let bot = Bot::new(config.get_token().to_string());
        return BotInstance {
            config,
            bot,
        }
    }

    pub fn get_bot(&self) -> Bot {
        return self.bot.clone();
    }
}