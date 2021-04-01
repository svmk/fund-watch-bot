use crate::console::command::run_command::RunCommand;
use crate::prelude::*;
use typed_di::service::service::Service;
use crate::telegram::task::telegram_bot_task::TelegramBotTask;

#[derive(new)]
pub struct RunTelegram {
    telegram_bot_task: Service<TelegramBotTask>,
}

impl RunTelegram {
    pub async fn run(&self, command: &RunCommand) -> Result<(), Failure> {
        return self.telegram_bot_task.run(command.get_started_at()).await;
    }
}