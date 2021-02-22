use crate::prelude::*;
use crate::event_emitter::model::event_record::EventRecord;
use crate::event_emitter::model::event::Event;
use futures::future::BoxFuture;
use std::future::Future;
use std::pin::Pin;

pub trait EventListener<P> where P: Event {
    fn handle_event(&self, event: EventRecord<P>) -> BoxFuture<Result<(), Failure>>;
}

impl <P, F, Fut> EventListener<P> for F 
    where
        P: Event,
        Fut: Future<Output=Result<(), Failure>> + Send + Sync + Unpin + 'static,
        F: Fn(EventRecord<P>) -> Fut {
            fn handle_event(&self, event: EventRecord<P>) -> BoxFuture<Result<(), Failure>> {
                let future = (self)(event);
                let future = Box::new(future);
                let future = Pin::new(future);
                return future;
            }
        }