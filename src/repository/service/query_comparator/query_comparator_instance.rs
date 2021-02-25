use crate::repository::service_handlers::query_comparator::QueryComparator;
use crate::repository::model::query::Query;
use crate::repository::service::query_comparator::packed_query_comparator::PackedQueryComparator;
use crate::prelude::*;
use std::any::{Any, type_name};

pub struct QueryComparatorInstance {
    comparator: Box<dyn PackedQueryComparator>,
}

impl QueryComparatorInstance {
    pub fn new<Q, E, C>(comparator: C) -> QueryComparatorInstance 
        where 
            C: QueryComparator<Q, E> + 'static,
            Q: Query,
            E: Any,
    {
        let comparator = move |query: &dyn Any, entity: &dyn Any| -> Result<bool, Failure> {
            let query = match query.downcast_ref::<Q>() {
                Some(query) => query,
                None => {
                    return crate::fail!("Unable to downcast ref query `{}` for query comparasion", type_name::<Q>());
                },
            };
            let entity = match entity.downcast_ref::<E>() {
                Some(entity) => entity,
                None => {
                    return crate::fail!("Unable to downcast ref entity `{}` for query comparasion", type_name::<E>());
                },
            };
            return comparator.compare_entity(query, entity);
        };
        let comparator: Box<dyn PackedQueryComparator> = Box::new(comparator);
        return QueryComparatorInstance {
            comparator,
        }
    }

    pub fn compare<Q, E>(&self, query: &Q, entity: &E) -> Result<bool, Failure> 
        where
            Q: Query,
            E: Any,
    {
        return self.comparator.compare_entity(entity, query);
    }
}