use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::event_category::EventCategory;
use std::sync::Arc;

#[derive(Clone)]
pub struct PackedEvent {
    event_category: EventCategory,
    payload: Arc<Box<dyn Event>>,
}

impl PackedEvent {
    pub fn new<P>(event_category: EventCategory, payload: P) -> PackedEvent where P: Event {
        return PackedEvent {
            event_category,
            payload: Arc::new(Box::new(payload)),
        }
    }

    pub fn get_event_category(&self) -> &EventCategory {
        return &self.event_category;
    }
}