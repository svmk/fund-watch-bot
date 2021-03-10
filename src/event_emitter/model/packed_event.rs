use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::event_record::EventRecord;
use crate::event_emitter::model::event_category::EventCategory;
use crate::prelude::*;
use std::any::{Any, type_name};
use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
pub struct PackedEvent {
    event_category: EventCategory,
    payload: Arc<Box<dyn Any + Send + Sync>>,
}

impl PackedEvent {
    pub fn new<P>(event_category: EventCategory, payload: P) -> PackedEvent 
        where 
            P: Event,
    {
        return PackedEvent {
            event_category,
            payload: Arc::new(Box::new(payload)),
        }
    }

    pub fn get_event_category(&self) -> &EventCategory {
        return &self.event_category;
    }

    pub fn create_event_record<P>(&self) -> Result<EventRecord<P>, Failure> 
        where 
            P: Event,
            P: Any,
            P: 'static,
    {
        let payload = match self.payload.downcast_ref::<P>() {
            Some(payload) => payload,
            None => {
                return crate::fail!("Unable to convert packed event `{}`", type_name::<P>());
            },
        };
        let event = EventRecord::new(self.event_category.clone(), payload.clone());
        return Ok(event);
    }
}

impl fmt::Debug for PackedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("event_category", &self.event_category)
            .finish()
    }
}