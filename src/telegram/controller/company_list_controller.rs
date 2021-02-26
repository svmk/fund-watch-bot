use crate::telegram::controller::prelude::*;

pub struct CompanyListController {

}


#[async_trait]
impl CommandHandler for CompanyListController {
    async fn handle_message(&self, message: IncomingMessage) -> Result<View, Failure> {
        unimplemented!()
    }
}