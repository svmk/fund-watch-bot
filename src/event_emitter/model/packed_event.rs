use crate::event_emitter::model::event::Event;

pub struct PackedEvent {
    payload: Box<dyn Event>,
}

impl PackedEvent {
    pub fn new<P>(payload: P) -> PackedEvent where P: Event {
        return PackedEvent {
            payload: Box::new(payload),
        }
    }
}