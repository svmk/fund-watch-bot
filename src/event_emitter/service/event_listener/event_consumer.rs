use crate::prelude::*;
use crate::event_emitter::model::event_record::EventRecord;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::packed_event::PackedEvent;
use futures::stream::Stream;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct EventConsumerSender {
    sender: UnboundedSender<PackedEvent>,
}

impl EventConsumerSender {
    pub fn new<T>() -> (EventConsumerSender, EventConsumer<T>) {
        let (sender, receiver) = unbounded_channel();
        let sender = EventConsumerSender {sender};
        let consumer = EventConsumer {
            receiver,
            _type: PhantomData {},
        };
        return (sender, consumer);
    }
    pub fn send(&self, event: PackedEvent) -> Result<(), Failure> {
        self.sender.send(event)?;
        return Ok(());
    }

    pub fn is_closed(&self) -> bool {
        return self.sender.is_closed();
    }
}

#[derive(Debug)]
pub struct EventConsumer<T> {
    receiver: UnboundedReceiver<PackedEvent>,
    _type: PhantomData<T>,
}

impl <T> Stream for EventConsumer<T> where T: Event {
    type Item = Result<EventRecord<T>, Failure>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut receiver = unsafe {
            self.map_unchecked_mut(|this| {
                return &mut this.receiver;
            })
        };
        match receiver.poll_recv(cx) {
            Poll::Pending => {
                return Poll::Pending;
            },
            Poll::Ready(None) => {
                return Poll::Ready(None);
            },
            Poll::Ready(Some(event)) => {
                let event = event.create_event_record::<T>();
                return Poll::Ready(Some(event));
            },
        }
    }
}