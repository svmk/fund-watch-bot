
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;
use crate::telegram::action::fund_report_list_action::FundReportListAction;
use crate::{repository::repository::file_repository::FileRepository, system::di, telegram::action::{fund_info_action::FundInfoAction, subscription_list_action::SubscriptionListAction}};
use crate::system::app_config::AppConfig;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::telegram::service::command_router::CommandRouter;
use crate::telegram::service::message_handler::MessageHandler;
use crate::telegram::service_handlers::command_handler::CommandHandler;
use crate::telegram::service_handlers::action_handler::ActionHandler;
use crate::telegram::action::fund_report_info_action::FundReportInfoAction;
use crate::telegram::service::action_router::ActionRouter;
use crate::telegram::service::bot_instance::BotInstance;
use crate::telegram::service::event_notifier::EventNotifier;
use crate::telegram::task::telegram_bot_task::TelegramBotTask;
use crate::telegram::action::fund_list_action::FundListAction;
use crate::telegram::action::fund_change_list_action::FundChangeListAction;
use crate::telegram::action::fund_change_info_action::FundChangeInfoAction;
use crate::telegram::model::action_id::ActionId;
use crate::telegram::controller::start_controller::StartController;
use crate::telegram::controller::fund_list_controller::FundListController;
use crate::telegram::controller::fund_info_controller::FundInfoController;
use crate::telegram::controller::subscription_list_controller::SubscriptionListController;
use crate::telegram::controller::fund_report_list_controller::FundReportListController;
use crate::telegram::controller::fund_report_info_controller::FundReportInfoController;
use crate::telegram::controller::fund_change_list_controller::FundChangeListController;
use crate::telegram::controller::fund_change_info_controller::FundChangeInfoController;
use crate::telegram::model::chat::Chat;
use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::chat_messages::ChatMessages;
use crate::telegram::model::command::Command;
use crate::telegram::model::action_type::ActionType;
use crate::telegram::path_resolver::chat_path_resolver::chat_path_resolver;
use crate::telegram::path_resolver::messages_path_resolver::messages_path_resolver;
use crate::telegram::path_resolver::action_id_path_resolver::action_id_path_resolver;
use crate::serializer::service::json_serializer::JsonSerializer;

pub const COMMAND_ROUTER: ServiceId<CommandRouter> = ServiceIdResolver::SERVICE_ID;
pub const MESSAGE_HANDLER: ServiceId<MessageHandler> = ServiceIdResolver::SERVICE_ID;
pub const ACTION_ROUTER: ServiceId<ActionRouter> = ServiceIdResolver::SERVICE_ID;
pub const BOT_INSTANCE: ServiceId<BotInstance> = ServiceIdResolver::SERVICE_ID;
pub const CHAT_REPOSITORY: ServiceId<RepositoryInstance<ChatId, Chat>> = ServiceIdResolver::SERVICE_ID;
pub const MESSAGES_REPOSITORY: ServiceId<RepositoryInstance<ChatId, ChatMessages>> = ServiceIdResolver::SERVICE_ID;
pub const EVENT_NOTIFIER: ServiceId<EventNotifier> = ServiceIdResolver::SERVICE_ID;
pub const TELEGRAM_BOT_TASK: ServiceId<TelegramBotTask> = ServiceIdResolver::SERVICE_ID;
pub const START_CONTROLLER: ServiceId<StartController> = ServiceIdResolver::SERVICE_ID;
pub const FUND_LIST_CONTROLLER: ServiceId<FundListController> = ServiceIdResolver::SERVICE_ID;
pub const FUND_LIST_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, FundListAction>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_REPORT_LIST_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, FundReportListAction>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_INFO_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, FundInfoAction>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_INFO_CONTROLLER: ServiceId<FundInfoController> = ServiceIdResolver::SERVICE_ID;
pub const SUBSCRIPTION_LIST_CONTROLLER: ServiceId<SubscriptionListController> = ServiceIdResolver::SERVICE_ID;
pub const SUBSCRIPTION_LIST_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, SubscriptionListAction>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_REPORT_LIST_CONTROLLER: ServiceId<FundReportListController> = ServiceIdResolver::SERVICE_ID;
pub const FUND_REPORT_INFO_CONTROLLER: ServiceId<FundReportInfoController> = ServiceIdResolver::SERVICE_ID;
pub const FUND_REPORT_INFO_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, FundReportInfoAction>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_CHANGE_LIST_CONTROLLER: ServiceId<FundChangeListController> = ServiceIdResolver::SERVICE_ID;
pub const FUND_CHANGE_LIST_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, FundChangeListAction>> = ServiceIdResolver::SERVICE_ID;
pub const FUND_CHANGE_INFO_CONTROLLER: ServiceId<FundChangeInfoController> = ServiceIdResolver::SERVICE_ID;
pub const FUND_CHANGE_INFO_ACTION_REPOSITORY: ServiceId<RepositoryInstance<ActionId, FundChangeInfoAction>> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(COMMAND_ROUTER, async move |resolver| {
        let start_controller = resolver.get_service(START_CONTROLLER).await?;
        let start_controller = typed_di::service_ref!(start_controller => &dyn CommandHandler);
        let fund_list_controller = resolver.get_service(FUND_LIST_CONTROLLER).await?;
        let fund_list_controller = typed_di::service_ref!(fund_list_controller => &dyn CommandHandler);
        let subscription_list_controller = resolver.get_service(SUBSCRIPTION_LIST_CONTROLLER).await?;
        let subscription_list_controller = typed_di::service_ref!(subscription_list_controller => &dyn CommandHandler);
        let mut service = CommandRouter::new();
        service.register_command(Command::Start, start_controller.clone());
        service.register_command(Command::Unknown, start_controller);
        service.register_command(Command::Funds, fund_list_controller);
        service.register_command(Command::Subscriptions, subscription_list_controller);
        return Ok(service);
    })?;
    builder.register(MESSAGE_HANDLER, async move |resolver| {
        let service = MessageHandler::new(
            resolver.get_service(COMMAND_ROUTER).await?,
            resolver.get_service(ACTION_ROUTER).await?,
            resolver.get_service(CHAT_REPOSITORY).await?,
            resolver.get_service(BOT_INSTANCE).await?,
        );
        return Ok(service);
    })?;
    builder.register(ACTION_ROUTER, async move |resolver| {
        let fund_list_controller = resolver.get_service(FUND_LIST_CONTROLLER).await?;
        let fund_list_controller = typed_di::service_ref!(fund_list_controller => &dyn ActionHandler);
        let fund_info_controller = resolver.get_service(FUND_INFO_CONTROLLER).await?;
        let fund_info_controller = typed_di::service_ref!(fund_info_controller => &dyn ActionHandler);
        let subscription_list_controller = resolver.get_service(SUBSCRIPTION_LIST_CONTROLLER).await?;
        let subscription_list_controller = typed_di::service_ref!(subscription_list_controller => &dyn ActionHandler);
        let fund_report_list_controller = resolver.get_service(FUND_REPORT_LIST_CONTROLLER).await?;
        let fund_report_list_controller = typed_di::service_ref!(fund_report_list_controller => &dyn ActionHandler);
        let fund_report_controller = resolver.get_service(FUND_REPORT_INFO_CONTROLLER).await?;
        let fund_report_controller = typed_di::service_ref!(fund_report_controller => &dyn ActionHandler);
        let fund_change_list_controller = resolver.get_service(FUND_CHANGE_LIST_CONTROLLER).await?;
        let fund_change_list_controller = typed_di::service_ref!(fund_change_list_controller => &dyn ActionHandler);
        let mut service = ActionRouter::new();
        service.register_action(ActionType::FUND_LIST, fund_list_controller);
        service.register_action(ActionType::FUND_INFO, fund_info_controller);
        service.register_action(ActionType::SUBSCRIPTION_LIST, subscription_list_controller);
        service.register_action(ActionType::FUND_REPORT_LIST, fund_report_list_controller);
        service.register_action(ActionType::FUND_REPORT_INFO, fund_report_controller);
        service.register_action(ActionType::FUND_CHANGE_LIST, fund_change_list_controller);
        return Ok(service);
    })?;
    builder.register(BOT_INSTANCE, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_bot_instance();
        let service = BotInstance::new(
            config,
            resolver.get_service(MESSAGES_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    builder.register(EVENT_NOTIFIER, async move |resolver| {
        let service = EventNotifier::new(
            resolver.get_service(BOT_INSTANCE).await?,
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(CHAT_REPOSITORY).await?,
            resolver.get_service(di::market_fund_report_di::FUND_CHANGES_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    builder.register(TELEGRAM_BOT_TASK, async move |resolver| {
        let service = TelegramBotTask::new(
            resolver.get_service(MESSAGE_HANDLER).await?,
            resolver.get_service(BOT_INSTANCE).await?,
        );
        return Ok(service);
    })?;
    builder.register(CHAT_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            chat_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(MESSAGES_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            messages_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(FUND_LIST_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(START_CONTROLLER, async move |_resolver| {
        let service = StartController::new();
        return Ok(service);
    })?;
    builder.register(FUND_LIST_CONTROLLER, async move |resolver| {
        let service = FundListController::new(
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(CHAT_REPOSITORY).await?,
            resolver.get_service(FUND_LIST_ACTION_REPOSITORY).await?,
            resolver.get_service(FUND_INFO_CONTROLLER).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_INFO_CONTROLLER, async move |resolver| {
        let service = FundInfoController::new(
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(CHAT_REPOSITORY).await?,
            resolver.get_service(FUND_INFO_ACTION_REPOSITORY).await?,
            resolver.get_service(FUND_REPORT_LIST_CONTROLLER).await?,
            resolver.get_service(FUND_CHANGE_LIST_CONTROLLER).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_INFO_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(SUBSCRIPTION_LIST_CONTROLLER, async move |resolver| {
        let service = SubscriptionListController::new(
            resolver.get_service(CHAT_REPOSITORY).await?,
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(SUBSCRIPTION_LIST_ACTION_REPOSITORY).await?,
            resolver.get_service(FUND_INFO_CONTROLLER).await?,
        );
        return Ok(service);
    })?;
    builder.register(SUBSCRIPTION_LIST_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(FUND_REPORT_LIST_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(FUND_REPORT_LIST_CONTROLLER, async move |resolver| {
        let service = FundReportListController::new(
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(di::market_fund_report_di::FUND_REPORTS_REPOSITORY).await?,
            resolver.get_service(FUND_REPORT_LIST_ACTION_REPOSITORY).await?,
            resolver.get_service(FUND_REPORT_INFO_CONTROLLER).await?,
        );       
        return Ok(service);
    })?;
    builder.register(FUND_REPORT_INFO_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(FUND_REPORT_INFO_CONTROLLER, async move |resolver| {
        let service = FundReportInfoController::new(
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(di::market_fund_report_di::DAILY_FUND_REPORT_REPOSITORY).await?,
            resolver.get_service(di::market_data_di::CANDLESTICK_PROVIDER).await?,
            resolver.get_service(FUND_REPORT_INFO_ACTION_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_CHANGE_LIST_CONTROLLER, async move |resolver| {
        let service = FundChangeListController::new(
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(di::market_fund_report_di::FUND_REPORTS_REPOSITORY).await?,
            resolver.get_service(FUND_CHANGE_LIST_ACTION_REPOSITORY).await?,
            resolver.get_service(FUND_CHANGE_INFO_CONTROLLER).await?,
        );
        return Ok(service);
    })?;
    builder.register(FUND_CHANGE_LIST_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(FUND_CHANGE_INFO_ACTION_REPOSITORY, async move |resolver| {
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            action_id_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(FUND_CHANGE_INFO_CONTROLLER, async move |resolver| {
        let service = FundChangeInfoController::new(
            resolver.get_service(di::market_fund_report_di::FUND_REPOSITORY).await?,
            resolver.get_service(di::market_fund_report_di::FUND_CHANGES_REPOSITORY).await?,
            resolver.get_service(di::market_data_di::CANDLESTICK_PROVIDER).await?,
            resolver.get_service(FUND_CHANGE_INFO_ACTION_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    return Ok(());
}