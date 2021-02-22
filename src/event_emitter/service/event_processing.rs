use crate::prelude::*;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::event_listener::EventListener;
use crate::event_emitter::model::event_category::EventCategory;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::model::packed_event_listener::PackedEventListener;
use std::collections::HashMap;
mod event_listener_item;
use self::event_listener_item::EventListenerItem;

#[derive(new)]
pub struct EventProcessing {
    event_listeners: HashMap<EventCategory, Vec<Box<dyn PackedEventListener>>>,
}

impl EventProcessing {
    pub fn add_event_listener<P, L>(&mut self, listener: L) 
        where 
            P: Event,
            L: EventListener<P>,
            L: Send + Sync + 'static,
        {
            self.add_packed_event_listener(
                L::event_category(), 
                EventListenerItem::new(listener)
            );
    }

    pub fn add_packed_event_listener<L>(&mut self, category: EventCategory, listener: L) 
        where 
            L: PackedEventListener,
            L: Send + Sync + 'static,
    {
        if !self.event_listeners.contains_key(&category) {
            let _ = self.event_listeners.insert(category.clone(), Vec::new());
        }
        let categories = self.event_listeners.get_mut(&category).unwrap();
        categories.push(Box::new(listener));
    }

    pub async fn emit_event(&self, event: PackedEvent) -> Result<(), Failure> 
    {
        if let Some(event_listeners) = self
            .event_listeners
            .get(event.get_event_category()) {
                for event_listener in event_listeners.iter() {
                    event_listener.handle_event(event.clone()).await?;
                }
        }
        return Ok(());
    }
}