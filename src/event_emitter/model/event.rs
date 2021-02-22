use crate::event_emitter::model::event_category::EventCategory;
use std::any::Any;
use std::any::TypeId;
pub trait Event: Any 
    where 
        Self: Clone,
        Self: Send + Sync,
{
    fn event_category() -> EventCategory where Self: Sized {
        let type_id = TypeId::of::<Self>();
        return EventCategory::from_type_id(type_id);
    }
}