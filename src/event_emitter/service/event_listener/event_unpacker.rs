use crate::prelude::*;
use crate::event_emitter::service_handler::event_handler::EventHandler;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service_handler::packed_event_handler::PackedEventHandler;

pub struct EventUnpacker<P> where P: Event {
    handler: Box<dyn EventHandler<P> + Send + Sync>,
}

impl <P>EventUnpacker<P> where P: Event {
    pub fn new<L>(handler: L) -> EventUnpacker<P> 
        where L: EventHandler<P> + Send + Sync + 'static, 
    {
        return EventUnpacker {
            handler: Box::new(handler),
        }
    }
}


#[async_trait]
impl <P>PackedEventHandler for EventUnpacker<P> where P: Event {
    async fn handle_event(&self, event: PackedEvent) -> Result<(), Failure> {
        let event = event.create_event_record::<P>()?;
        return self.handler.handle_event(event).await;
    }
}