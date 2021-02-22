use crate::prelude::*;
use crate::event_emitter::model::event_record::EventRecord;
use crate::event_emitter::model::event::Event;
use futures::future::BoxFuture;
use std::future::Future;
use std::pin::Pin;

use super::event_category::EventCategory;

pub trait EventListener<P>: Send + Sync where P: Event {
    fn handle_event(&self, event: EventRecord<P>) -> BoxFuture<Result<(), Failure>>;
    fn event_category() -> EventCategory where Self: Sized {
        return P::event_category();
    }
}

impl <P, F, Fut> EventListener<P> for F 
    where
        P: Event,
        Fut: Future<Output=Result<(), Failure>> + Send + Sync + Unpin + 'static,
        F: Fn(EventRecord<P>) -> Fut,
        F: Send + Sync,
        {
            fn handle_event(&self, event: EventRecord<P>) -> BoxFuture<Result<(), Failure>> {
                let future = (self)(event);
                let future = Box::new(future);
                let future = Pin::new(future);
                return future;
            }
        }