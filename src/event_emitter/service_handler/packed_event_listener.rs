use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use futures::future::BoxFuture;

pub trait PackedEventListener: Send + Sync {
    fn handle_event(&self, event: PackedEvent) -> BoxFuture<Result<(), Failure>>;
}

impl <F> PackedEventListener for F 
    where
        F: Fn(PackedEvent) -> BoxFuture<'static, Result<(), Failure>>,
        F: Send + Sync,
        {
            fn handle_event(&self, event: PackedEvent) -> BoxFuture<Result<(), Failure>> {
                return (self)(event);
            }
        }