use crate::console::command::import_13f_form_command::Import13FFormCommand;

#[derive(Debug, StructOpt)]
pub enum ConsoleCommand {
    Import13FForm(Import13FFormCommand),
}