use crate::event_emitter::model::event_handler_id::EventHandlerId;
use crate::sec_gov::events::edgar_cache_access_event::EdgarCacheAccessEvent;

pub const WATCH_EDGAR_CACHE_ACCESS: EventHandlerId<EdgarCacheAccessEvent> = EventHandlerId::from_uuid(0x105d1c67_aea5_48a6_8f5f_3a06af278b1a);