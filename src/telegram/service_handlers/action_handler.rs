use crate::prelude::*;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::view::View;
use std::future::Future;

#[async_trait]
pub trait ActionHandler: Send + Sync {
    async fn handle_message(&self, context: &ChatContext, message: ActionRoute) -> Result<View, Failure>;
}

#[async_trait]
impl <F, Fut>ActionHandler for F 
    where 
        F: Fn(&ChatContext, ActionRoute) -> Fut,
        F: Send + Sync,
        Fut: Future<Output=Result<View, Failure>> + Send + Sync + 'static,
        {
            async fn handle_message(&self, context: &ChatContext, message: ActionRoute) -> Result<View, Failure> 
            {
                let future = (self)(context, message);
                return future.await;
            }
        }