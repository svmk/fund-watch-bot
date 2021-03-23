use crate::prelude::*;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::model::chat_context::ChatContext;
use crate::telegram::model::view::View;
use typed_di::service::service::Service;
use std::ops::Deref;
use std::future::Future;

#[async_trait]
pub trait ActionHandler: Send + Sync {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure>;
}

#[async_trait]
impl <F, Fut>ActionHandler for F 
    where 
        F: Fn(&ChatContext, ActionRoute) -> Fut,
        F: Send + Sync,
        Fut: Future<Output=Result<View, Failure>> + Send + Sync + 'static,
        {
            async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> 
            {
                let future = (self)(context, action_route);
                return future.await;
            }
        }

#[async_trait]
impl ActionHandler for Service<&dyn ActionHandler> {
    async fn handle_action(&self, context: &ChatContext, action_route: ActionRoute) -> Result<View, Failure> {
        return self.deref().handle_action(context, action_route).await;
    }
}