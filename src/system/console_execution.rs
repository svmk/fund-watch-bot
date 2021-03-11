use crate::prelude::*;
use crate::console::console_application::{ConsoleApplication, ConsoleCommand};
use crate::system::di::console_di;
use typed_di::async_di::container::Container;

pub async fn execute_console(container: Container, config: &ConsoleApplication) -> Result<(), Failure> {
    match config.command {
        ConsoleCommand::Import13FForm(ref command) => {
            let service = container.get_service(console_di::IMPORT_13F_FORM_SERVICE).await?;
            return service.run(command).await;
        },
    }
    return Ok(());
}