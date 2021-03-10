use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use std::future::Future;


#[async_trait]
pub trait PackedEventHandler: Send + Sync {
    async fn handle_event(&self, event: PackedEvent) -> Result<(), Failure>;
}


#[async_trait]
impl <F, Fut> PackedEventHandler for F 
    where
        F: Fn(PackedEvent) -> Fut,
        Fut: Future<Output=Result<(), Failure>> + Send + 'static,
        F: Send + Sync,
        {
            async fn handle_event(&self, event: PackedEvent) -> Result<(), Failure> {
                return (self)(event).await;
            }
        }