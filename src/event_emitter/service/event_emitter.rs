use crate::prelude::*;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service::event_listener::EventListener;
use typed_di::service::service::Service;

#[derive(new)]
pub struct EventEmitter {
    event_listener: Service<EventListener>,
}

impl EventEmitter {
    pub async fn emit_event<E>(&self, payload: E) -> Result<(), Failure> 
        where E: Event
    {
        let event = PackedEvent::new(E::event_category(), payload);
        return self.event_listener.emit_event(event).await;
    }
}