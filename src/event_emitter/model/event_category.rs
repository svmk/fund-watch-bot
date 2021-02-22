use std::any::TypeId;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EventCategory {
    type_id: TypeId,
}

impl EventCategory {
    pub fn from_type_id(type_id: TypeId) -> EventCategory {
        return EventCategory {
            type_id,
        }
    }
}