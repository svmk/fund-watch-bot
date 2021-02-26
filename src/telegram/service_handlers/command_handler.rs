use crate::prelude::*;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::view::View;
use std::future::Future;

#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_message(&self, context: &ChatContext, message: IncomingMessage) -> Result<View, Failure>;
}

#[async_trait]
impl <F, Fut>CommandHandler for F 
    where 
        F: Fn(&ChatContext, IncomingMessage) -> Fut,
        F: Send + Sync,
        Fut: Future<Output=Result<View, Failure>> + Send + Sync + 'static,
        {
            async fn handle_message(&self, context: &ChatContext, message: IncomingMessage) -> Result<View, Failure> 
            {
                let future = (self)(context, message);
                return future.await;
            }
        }