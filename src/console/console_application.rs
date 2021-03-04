use std::path::PathBuf;
mod console_command;
pub use crate::console::console_application::console_command::ConsoleCommand;

#[derive(Debug, StructOpt)]
#[structopt(name = "fund-watch-bot", about = "Telegram bot for watching fund activity.")]
pub struct ConsoleApplication {
    #[structopt(parse(from_os_str))]
    pub config_path: PathBuf,
    #[structopt(subcommand)]
    pub command: ConsoleCommand,
}