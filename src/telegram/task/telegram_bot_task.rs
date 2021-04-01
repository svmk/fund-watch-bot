use crate::app::model::date::Date;
use crate::{prelude::*};
use crate::telegram::service::message_handler::MessageHandler;
use crate::telegram::service::bot_instance::BotInstance;
use crate::telegram::service::event_notifier::EventNotifier;
use crate::telegram::event_handlers::FUND_CHANGE_TELEGRAM_NOTIFICATION;
use crate::market::fund_report::model::daily_fund_report_import_request::DailyFundReportImportRequest;
use crate::telegram::views::bot_command_settings_view::bot_command_settings_view;
use crate::market::fund_report::service::daily_fund_report_importing::DailyFundReportImporting;
use crate::event_emitter::service::event_listener::EventListener;
use sentry::integrations::anyhow::capture_anyhow as sentry_capture_error;
use typed_di::service::service::Service;
use tbot::types::update::Kind;
use async_std::task::sleep;
mod telegram_bot_task_config;
pub use self::telegram_bot_task_config::TelegramBotTaskConfig;
use futures::future::try_join3;


#[derive(new)]
pub struct TelegramBotTask {
    config: TelegramBotTaskConfig,
    message_handler: Service<MessageHandler>,
    bot_instance: Service<BotInstance>,
    daily_fund_report_importing: Service<DailyFundReportImporting>,
    event_listener: Service<EventListener>,
    event_notifier: Service<EventNotifier>,
}

impl TelegramBotTask {
    pub async fn run(&self, started_at: Option<Date>) -> Result<(), Failure> {
        let events_future = self.run_events();
        let import_future = self.run_import(started_at);
        let run_bot = self.run_bot();
        try_join3(events_future, import_future, run_bot).await?;
        return Ok(());
    }
    
    async fn run_events(&self) -> Result<(), Failure> {
        let event_notifier = self.event_notifier.clone();
        let handle = self
            .event_listener
            .listen(FUND_CHANGE_TELEGRAM_NOTIFICATION)
            .within_receiver_context(move |event| {
                let event_notifier = event_notifier.clone();
                return event_notifier.handle_new_fund_change_event(event);
            }).await?;
        handle.await?;
        return Ok(());
    }

    async fn run_import(&self, started_at: Option<Date>) -> Result<(), Failure> {
        loop {
            let start_at = started_at.clone().unwrap_or(Date::today());
            let request = DailyFundReportImportRequest::new(start_at);
            self.daily_fund_report_importing.import_period(request).await?;
            sleep(self.config.get_import_delay().clone()).await;
        }
    }

    async fn run_bot(&self) -> Result<(), Failure> {
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
                    let sentry_uuid = sentry_capture_error(&error);
                    eprintln!("Telegram error: {}. Sentry uuid = {}", error, sentry_uuid);
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