use crate::console::command::import_13f_form_command::Import13FFormCommand;
use crate::console::command::run_command::RunCommand;

#[derive(Debug, StructOpt)]
pub enum ConsoleCommand {
    #[structopt(name = "import-13f-form", about = "Imports 13f form from sec.gov site.")]
    Import13FForm(Import13FFormCommand),
    #[structopt(name = "run", about = "Executes telegram bot.")]
    Run(RunCommand),
}