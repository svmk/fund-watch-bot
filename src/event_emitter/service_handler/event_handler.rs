use crate::prelude::*;
use crate::event_emitter::model::event_record::EventRecord;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::event_category::EventCategory;
use std::future::Future;

#[async_trait]
pub trait EventHandler<P>: Send + Sync where P: Event {
    async fn handle_event(&self, event: EventRecord<P>) -> Result<(), Failure>;
    fn event_category() -> EventCategory where Self: Sized {
        return P::event_category();
    }
}


#[async_trait]
impl <P, F, Fut> EventHandler<P> for F 
    where
        P: Event,
        Fut: Future<Output=Result<(), Failure>> + Send + Sync + Unpin + 'static,
        F: Fn(EventRecord<P>) -> Fut,
        F: Send + Sync,
        {
            async fn handle_event(&self, event: EventRecord<P>) -> Result<(), Failure> {
                return (self)(event).await;
            }
        }