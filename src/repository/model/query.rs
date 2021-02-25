use std::any::{Any, TypeId};
use crate::repository::model::query_type::QueryType;

pub trait Query: Any {
    fn get_query_type() -> QueryType {
        let typed_id = TypeId::of::<Self>();
        return QueryType::new(typed_id);
    }
}