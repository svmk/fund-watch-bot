use crate::prelude::*;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service::event_processing::EventProcessing;
use typed_di::service::Service;

#[derive(new)]
pub struct EventEmitter {
    event_processing: Service<EventProcessing>,
}

impl EventEmitter {
    pub async fn emit_event<E>(&self, payload: E) -> Result<(), Failure> 
        where E: Event
    {
        let event = PackedEvent::new(payload);
        return self.event_processing.emit_event(event).await;
    }
}