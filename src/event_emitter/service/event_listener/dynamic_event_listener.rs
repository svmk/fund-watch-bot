use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service::event_listener::event_consumer::EventConsumerSender;
use crate::event_emitter::service::event_listener::event_executor::EventExecutor;
use crate::event_emitter::service::event_listener::event_handler::EventProcessingSender;

pub enum DynamicEventListener {
    EventConsumer(EventConsumerSender),
    EventExecutor(EventExecutor),
    EventProcessing(EventProcessingSender),
}

impl DynamicEventListener {
    pub fn is_closed(&self) -> bool {
        match self {
            DynamicEventListener::EventConsumer(service) => {
                return service.is_closed();
            },
            DynamicEventListener::EventExecutor(service) => {
                return service.is_closed();
            },
            DynamicEventListener::EventProcessing(service) => {
                return service.is_closed();
            },
        }
    }

    pub async fn handle_event(&self, event: PackedEvent) -> Result<(), Failure> {
        match self {
            DynamicEventListener::EventConsumer(service) => {
                return service.send(event);
            },
            DynamicEventListener::EventExecutor(service) => {
                return service.execute(event).await;
            },
            DynamicEventListener::EventProcessing(service) => {
                return service.send(event);
            },
        }
    }
}