use crate::event_emitter::model::event_category::EventCategory;
use std::any::Any;
use std::any::TypeId;
pub trait Event: Any {
    fn event_category(&self) -> EventCategory {
        let type_id = TypeId::of::<Self>();
        return EventCategory::from_type_id(type_id);
    }
}