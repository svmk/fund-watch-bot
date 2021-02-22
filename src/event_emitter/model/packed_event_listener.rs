use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use futures::future::BoxFuture;
use std::future::Future;
use std::pin::Pin;

pub trait PackedEventListener {
    fn handle_event(&self, event: PackedEvent) -> BoxFuture<Result<(), Failure>>;
}

impl <F, Fut> PackedEventListener for F 
    where
        Fut: Future<Output=Result<(), Failure>> + Send + Sync + Unpin + 'static,
        F: Fn(PackedEvent) -> Fut {
            fn handle_event(&self, event: PackedEvent) -> BoxFuture<Result<(), Failure>> {
                let future = (self)(event);
                let future = Box::new(future);
                let future = Pin::new(future);
                return future;
            }
        }