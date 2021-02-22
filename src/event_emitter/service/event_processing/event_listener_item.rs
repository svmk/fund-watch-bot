use crate::prelude::*;
use crate::event_emitter::model::event_listener::EventListener;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::model::packed_event_listener::PackedEventListener;
use futures::future::{BoxFuture, FutureExt};

pub struct EventListenerItem<P> where P: Event {
    listener: Box<dyn EventListener<P> + Send + Sync>,
}

impl <P>EventListenerItem<P> where P: Event {
    pub fn new<L>(listener: L) -> EventListenerItem<P> 
        where L: EventListener<P> + Send + Sync + 'static, 
    {
        return EventListenerItem {
            listener: Box::new(listener),
        }
    }
}

impl <P>PackedEventListener for EventListenerItem<P> where P: Event {
    fn handle_event(&self, event: PackedEvent) -> BoxFuture<Result<(), Failure>> {
        let event = event.create_event_record::<P>();
        let event = match event {
            Ok(event) => event,
            Err(error) => {
                return futures::future::err(error).boxed();
            },
        };
        let future = self.listener.handle_event(event);
        return future.boxed();
    }
}