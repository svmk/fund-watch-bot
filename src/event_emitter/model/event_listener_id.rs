#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventListenerId(u128);

impl EventListenerId {
    pub const fn new(uuid: u128) -> EventListenerId {
        return EventListenerId(uuid);
    }
}