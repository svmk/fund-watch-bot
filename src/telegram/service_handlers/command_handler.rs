use crate::prelude::*;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::view::View;
use std::future::Future;

#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_message(&self, message: IncomingMessage) -> Result<View, Failure>;
}

#[async_trait]
impl <F, Fut>CommandHandler for F 
    where 
        F: Fn(IncomingMessage) -> Fut,
        F: Send + Sync,
        Fut: Future<Output=Result<View, Failure>> + Send + Sync + 'static,
        {
            async fn handle_message(&self, message: IncomingMessage) -> Result<View, Failure> 
            {
                let future = (self)(message);
                return future.await;
            }
        }