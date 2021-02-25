use crate::prelude::*;
use std::any::Any;
use crate::repository::model::query::Query;

pub trait QueryComparator<E, Q>: Send + Sync
    where 
        E: Any,
        Q: Query,
{
    fn compare_entity(&self, entity: &E, query: &Q) -> Result<bool, Failure>;
}