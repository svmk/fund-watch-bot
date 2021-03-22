use crate::telegram::controller::prelude::*;
use crate::telegram::views::start_view::start_view;

#[derive(new)]
pub struct StartController {

}

#[async_trait]
impl CommandHandler for StartController {
    async fn handle_message(&self, _context: &ChatContext, _message: IncomingMessage) -> Result<View, Failure> {
        let view = start_view();
        return Ok(view);
    }
}