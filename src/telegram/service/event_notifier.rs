use crate::telegram::service::bot_instance::BotInstance;
use typed_di::service::Service;

#[derive(new)]
pub struct EventNotifier {
    bot_instance: Service<BotInstance>,
}

