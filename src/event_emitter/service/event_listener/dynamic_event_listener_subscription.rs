use crate::prelude::*;
use crate::event_emitter::service::event_listener::EventListener;
use crate::event_emitter::service::event_listener::dynamic_event_listener::DynamicEventListener;
use crate::event_emitter::service::event_listener::event_consumer::{EventConsumer, EventConsumerSender};
use crate::event_emitter::service::event_listener::event_executor::{EventExecutor, EventExecutorHandle};
use crate::event_emitter::service::event_listener::event_handler::{EventProcessing, EventProcessingSender};
use crate::event_emitter::service::event_listener::event_unpacker::EventUnpacker;
use crate::event_emitter::service_handler::event_handler::EventHandler;
use crate::event_emitter::model::event_category::EventCategory;
use crate::event_emitter::model::raw_event_handler_id::RawEventHandlerId;
use crate::event_emitter::model::event::Event;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct DynamicEventListenerSubscription<'a, E> {
    event_handler: &'a EventListener,
    event_category: EventCategory,
    event_handler_id: RawEventHandlerId,
    _type: PhantomData<E>,
}

impl <'a, E>DynamicEventListenerSubscription<'a, E> 
    where E: Event 
{
    pub fn new(
        event_handler: &'a EventListener,
        event_category: EventCategory,
        event_handler_id: RawEventHandlerId,
    ) -> DynamicEventListenerSubscription<'a, E> {
        return DynamicEventListenerSubscription {
            event_handler,
            event_category,
            event_handler_id,
            _type: PhantomData{},
        }
    }
    pub async fn consume(&self) -> Result<EventConsumer<E>, Failure> {
        let mut handlers = self
            .event_handler
            .dynamic_event_handlers
            .write().await;
        if handlers.get(&self.event_category).is_none() {
            let _ = handlers.insert(self.event_category.clone(), HashMap::new());
        }
        let handlers = handlers
            .get_mut(&self.event_category)
            .unwrap();
        if let Some(handler) = handlers.get(&self.event_handler_id) {
            if !handler.is_closed() {
                return crate::fail!("Already subscribed to listener with id `{:?}`", self.event_handler_id);
            }
        }
        let (consumer_sender, consumer_listener) = EventConsumerSender::new::<E>();
        let _ = handlers.insert(self.event_handler_id.clone(), DynamicEventListener::EventConsumer(consumer_sender));
        return Ok(consumer_listener);
    }

    pub async fn within_sender_context<L>(&self, handler: L) -> Result<EventExecutorHandle, Failure> 
        where 
            L: EventHandler<E> + Send + Sync + 'static,
    {
        let mut handlers = self
            .event_handler
            .dynamic_event_handlers
            .write().await;
        if handlers.get(&self.event_category).is_none() {
            let _ = handlers.insert(self.event_category.clone(), HashMap::new());
        }
        let handlers = handlers
            .get_mut(&self.event_category)
            .unwrap();
        if let Some(handler) = handlers.get(&self.event_handler_id) {
            if !handler.is_closed() {
                return crate::fail!("Already subscribed to listener with id `{:?}`", self.event_handler_id);
            }
        }
        let handler = EventUnpacker::new(handler);
        let handler = Box::new(handler);
        let (event_executor, event_executor_handle) = EventExecutor::new(handler);
        let _ = handlers.insert(self.event_handler_id.clone(), DynamicEventListener::EventExecutor(event_executor));
        return Ok(event_executor_handle);
    }

    pub async fn within_receiver_context<L>(&self, handler: L) -> Result<EventProcessing, Failure> 
        where 
            L: EventHandler<E> + Send + Sync + 'static,
    {
        let mut handlers = self
            .event_handler
            .dynamic_event_handlers
            .write().await;
        if handlers.get(&self.event_category).is_none() {
            let _ = handlers.insert(self.event_category.clone(), HashMap::new());
        }
        let handlers = handlers
            .get_mut(&self.event_category)
            .unwrap();
        if let Some(handler) = handlers.get(&self.event_handler_id) {
            if !handler.is_closed() {
                return crate::fail!("Already subscribed to listener with id `{:?}`", self.event_handler_id);
            }
        }
        let handler = EventUnpacker::new(handler);
        let handler = Box::new(handler);
        let (event_processing, event_processing_handle) = EventProcessingSender::new(handler);
        let _ = handlers.insert(self.event_handler_id.clone(), DynamicEventListener::EventProcessing(event_processing));
        return Ok(event_processing_handle);
    }
}