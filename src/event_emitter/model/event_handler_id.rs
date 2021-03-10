use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::raw_event_handler_id::RawEventHandlerId;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventHandlerId<T> {
    value: RawEventHandlerId,
    _type: PhantomData<T>,
}

impl <T>EventHandlerId<T> 
    where 
        T: Event,
{
    pub const fn from_uuid(uuid: u128) -> EventHandlerId<T> {
        return EventHandlerId {
            value: RawEventHandlerId::from_u128(uuid),
            _type: PhantomData{}
        }
    }

    pub fn get_raw_id(&self) -> RawEventHandlerId {
        return self.value.clone();
    }
}