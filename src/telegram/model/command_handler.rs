use futures::future::{BoxFuture, FutureExt};
use crate::prelude::*;
use crate::telegram::model::incoming_message::IncomingMessage;
use crate::telegram::model::view::View;
use std::future::Future;
pub trait CommandHandler: Send + Sync {
    fn handle_message(&self, message: IncomingMessage) -> BoxFuture<Result<View, Failure>>;
}

impl <F, Fut>CommandHandler for F 
    where 
        F: Fn(IncomingMessage) -> Fut,
        F: Send + Sync,
        Fut: Future<Output=Result<View, Failure>> + Send + Sync + 'static,
        {
            fn handle_message(&self, message: IncomingMessage) -> BoxFuture<Result<View, Failure>> {
                let future = (self)(message);
                let future = future.boxed();
                return future;
            }
        }