use crate::prelude::*;
use crate::event_emitter::model::event_record::EventRecord;
use crate::event_emitter::model::event::Event;
use futures::future::BoxFuture;

pub trait PackedEventListener<P> where P: Event {
    fn handle_event(&self, event: EventRecord<P>) -> BoxFuture<Result<(), Failure>>;
}
