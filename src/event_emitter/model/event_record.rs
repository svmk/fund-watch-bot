#[derive(Debug, Clone)]
pub struct EventRecord<P> {
    payload: P,
}

impl <P>EventRecord<P> {
    pub fn new(payload: P) -> EventRecord<P> {
        return EventRecord {
            payload,
        }
    }

    pub fn get_payload(&self) -> &P {
        return &self.payload;
    }
}