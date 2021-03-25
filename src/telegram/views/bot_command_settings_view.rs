use crate::telegram::model::bot_command_settings::BotCommandSettings;
use crate::telegram::model::command::Command;

pub fn bot_command_settings_view() -> BotCommandSettings {
    let mut result = BotCommandSettings::new();
    result.register_description(Command::Start, "Перейти к приветственному сообщению.");
    result.register_description(Command::Funds, "Список всех доступных фондов.");
    result.register_description(Command::Subscriptions, "Список фондов, на которые вы подписаны.");
    return result;
}