use crate::prelude::*;
use std::any::Any;
use crate::repository::model::query::Query;

pub trait QueryComparator<Q, E>: Send + Sync
    where 
        Q: Query,
        E: Any,
{
    fn compare_entity(&self, query: &Q, entity: &E) -> Result<bool, Failure>;
}

impl <F, Q, E>QueryComparator<Q, E> for F 
    where 
        Q: Query,
        E: Any,
        F: Fn(&Q, &E) -> Result<bool, Failure>,
        F: Send + Sync,
    {
        fn compare_entity(&self, query: &Q, entity: &E) -> Result<bool, Failure> {
            return (self)(query, entity);
        }
    }