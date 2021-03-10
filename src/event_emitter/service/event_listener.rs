use crate::prelude::*;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::event_handler_id::EventHandlerId;
use crate::event_emitter::model::raw_event_handler_id::RawEventHandlerId;
use crate::event_emitter::service_handler::event_handler::EventHandler;
use crate::event_emitter::model::event_category::EventCategory;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service_handler::packed_event_handler::PackedEventHandler;
use std::collections::HashMap;
mod event_unpacker;
use self::event_unpacker::EventUnpacker;
mod dynamic_event_listener;
use self::dynamic_event_listener::DynamicEventListener;
mod event_consumer;
pub use self::event_consumer::EventConsumer;
mod event_handler;
pub use self::event_handler::EventProcessing;
mod event_executor;
pub use self::event_executor::EventExecutor;
mod dynamic_event_listener_subscription;
pub use self::dynamic_event_listener_subscription::DynamicEventListenerSubscription;
use tokio::sync::RwLock;

pub struct EventListener {
    static_event_handlers: HashMap<EventCategory, Vec<Box<dyn PackedEventHandler>>>,
    dynamic_event_handlers: RwLock<HashMap<EventCategory, HashMap<RawEventHandlerId, DynamicEventListener>>>,
}

impl EventListener {
    pub fn new() -> EventListener {
        return EventListener {
            static_event_handlers: HashMap::new(),
            dynamic_event_handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn listen<P>(&self, handler_id: EventHandlerId<P>) -> DynamicEventListenerSubscription<'_, P>
        where P: Event 
    {
        return DynamicEventListenerSubscription::new(
            &self,
            P::event_category(),
            handler_id.get_raw_id(),
        );
    }
    
    pub fn register_static_listener<P, L>(&mut self, handler: L) 
        where 
            P: Event,
            L: EventHandler<P>,
            L: Send + Sync + 'static,
        {
            let category = L::event_category();
            let handler = EventUnpacker::new(handler);
            if !self.static_event_handlers.contains_key(&category) {
                let _ = self.static_event_handlers.insert(category.clone(), Vec::new());
            }
            let categories = self.static_event_handlers.get_mut(&category).unwrap();
            categories.push(Box::new(handler));
    }

    pub async fn emit_event(&self, event: PackedEvent) -> Result<(), Failure> 
    {
        if let Some(event_handlers) = self
            .static_event_handlers
            .get(event.get_event_category()) {
                for event_handler in event_handlers.iter() {
                    event_handler.handle_event(event.clone()).await?;
                }
        }

        let dynamic_event_handlers = self
            .dynamic_event_handlers
            .read()
            .await;
        let mut need_cleanup = false;
        if let Some(event_handlers) = dynamic_event_handlers.get(event.get_event_category()) {
                for event_handler in event_handlers.values() {
                    if event_handler.is_closed() {
                        need_cleanup = true;
                    } else {
                        event_handler.handle_event(event.clone()).await?;
                    }
                }
        }
        if need_cleanup {
            self.cleanup().await;
        }
        return Ok(());
    }

    async fn cleanup(&self) {
        let mut event_handlers = self
            .dynamic_event_handlers
            .write().await;
        for category_event_handlers in event_handlers.values_mut() {
            let iterator = category_event_handlers.drain_filter(|_, event_listener| {
                return event_listener.is_closed();
            });
            for _ in iterator {}
        }
    }
}