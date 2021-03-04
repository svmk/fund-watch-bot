use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::system::di;
use crate::system::app_config::AppConfig;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::service::command_router::CommandRouter;
use crate::telegram::service::message_handler::MessageHandler;
use crate::telegram::service::action_router::ActionRouter;
use crate::telegram::service::bot_instance::BotInstance;
use crate::telegram::service::event_notifier::EventNotifier;
use crate::telegram::task::telegram_bot_task::TelegramBotTask;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat_messages::ChatMessages;

pub const COMMAND_ROUTER: ServiceId<CommandRouter> = ServiceIdResolver::SERVICE_ID;
pub const MESSAGE_HANDLER: ServiceId<MessageHandler> = ServiceIdResolver::SERVICE_ID;
pub const ACTION_ROUTER: ServiceId<ActionRouter> = ServiceIdResolver::SERVICE_ID;
pub const BOT_INSTANCE: ServiceId<BotInstance> = ServiceIdResolver::SERVICE_ID;
pub const CHAT_REPOSITORY: ServiceId<RepositoryInstance<ChatId, Chat>> = ServiceIdResolver::SERVICE_ID;
pub const MESSAGES_REPOSITORY: ServiceId<RepositoryInstance<ChatId, ChatMessages>> = ServiceIdResolver::SERVICE_ID;
pub const EVENT_NOTIFIER: ServiceId<EventNotifier> = ServiceIdResolver::SERVICE_ID;
pub const TELEGRAM_BOT_TASK: ServiceId<TelegramBotTask> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(COMMAND_ROUTER, |resolver| {
        let service = CommandRouter::new();
        return Ok(service);
    })?;
    builder.register(MESSAGE_HANDLER, |resolver| {
        let service = MessageHandler::new(
            resolver.get_service(COMMAND_ROUTER)?,
            resolver.get_service(ACTION_ROUTER)?,
            resolver.get_service(CHAT_REPOSITORY)?,
            resolver.get_service(BOT_INSTANCE)?,
        );
        return Ok(service);
    })?;
    builder.register(ACTION_ROUTER, |_resolver| {
        let service = ActionRouter::new();
        return Ok(service);
    })?;
    builder.register(BOT_INSTANCE, |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_bot_instance();
        let service = BotInstance::new(
            config,
            resolver.get_service(MESSAGES_REPOSITORY)?,
        );
        return Ok(service);
    })?;
    builder.register(EVENT_NOTIFIER, |resolver| {
        let service = EventNotifier::new(
            resolver.get_service(BOT_INSTANCE)?,
            resolver.get_service(CHAT_REPOSITORY)?,
            resolver.get_service(di::market_fund_report_di::FUND_CHANGES_REPOSITORY)?,
        );
        return Ok(service);
    })?;
    builder.register(TELEGRAM_BOT_TASK, |resolver| {
        let service = TelegramBotTask::new(
            resolver.get_service(MESSAGE_HANDLER)?,
            resolver.get_service(BOT_INSTANCE)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}