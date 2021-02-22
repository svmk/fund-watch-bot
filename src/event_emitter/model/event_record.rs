use crate::event_emitter::model::event_category::EventCategory;

#[derive(Debug, Clone)]
pub struct EventRecord<P> {
    category: EventCategory,
    payload: P,
}

impl <P>EventRecord<P> {
    pub fn new(category: EventCategory, payload: P) -> EventRecord<P> {
        return EventRecord {
            category,
            payload,
        }
    }

    pub fn get_payload(&self) -> &P {
        return &self.payload;
    }
}