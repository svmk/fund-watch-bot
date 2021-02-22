use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use futures::future::BoxFuture;

pub trait PackedEventListener {
    fn handle_event(&self, event: PackedEvent) -> BoxFuture<Result<(), Failure>>;
}
