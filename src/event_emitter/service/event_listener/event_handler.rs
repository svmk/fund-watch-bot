use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service_handler::packed_event_handler::PackedEventHandler;
use std::future::Future;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

#[derive(Debug)]
pub struct EventProcessingSender {
    sender: UnboundedSender<PackedEvent>,
}

impl EventProcessingSender {
    pub fn new(event_listener: Box<dyn PackedEventHandler>) -> (EventProcessingSender, EventProcessing) {
        let (sender, receiver) = unbounded_channel();
        let future = EventProcessing::handle_event(receiver, event_listener);
        let future = Box::pin(future);    
        let event_sender = EventProcessingSender {
            sender,
        };
        let event_processing = EventProcessing {
            future,
        };
        return (event_sender, event_processing);
    }

    pub fn is_closed(&self) -> bool {
        return self.sender.is_closed();
    }

    pub fn send(&self, event: PackedEvent) -> Result<(), Failure> {
        self.sender.send(event)?;
        return Ok(());
    }
}

pub struct EventProcessing {
    future: Pin<Box<dyn Future<Output=Result<(), Failure>>>>,
}

impl EventProcessing {

    async fn handle_event(mut receiver: UnboundedReceiver<PackedEvent>, event_listener: Box<dyn PackedEventHandler>) -> Result<(), Failure> {
        while let Some(event) = receiver.recv().await {
            event_listener.handle_event(event).await?;
        }
        return Ok(());
    }
}

impl Future for EventProcessing {
    type Output = Result<(), Failure>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let future = unsafe {
            self.map_unchecked_mut(|this| {
                return &mut this.future;
            })
        };
        return future.poll(cx);
    }
}