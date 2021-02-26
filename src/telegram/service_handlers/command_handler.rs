use futures::future::{BoxFuture, FutureExt};
use crate::prelude::*;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::view::View;
use std::future::Future;

#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_message(&self, message: IncomingMessage) -> Result<View, Failure>;
}

impl <F, Fut>CommandHandler for F 
    where 
        F: Fn(IncomingMessage) -> Fut,
        F: Send + Sync,
        Fut: Future<Output=Result<View, Failure>> + Send + Sync + 'static,
        {
            fn handle_message<'life0, 'async_trait>(&'life0 self, message: IncomingMessage) -> BoxFuture<'async_trait, Result<View, Failure>> 
                where
                'life0: 'async_trait,
            {
                let future = (self)(message);
                let future = future.boxed();
                return future;
            }
        }