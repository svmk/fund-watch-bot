use crate::repository::service_handlers::query_comparator::QueryComparator;
use crate::repository::model::query::Query;
use crate::repository::service::query_comparator::packed_query_comparator::PackedQueryComparator;
use crate::prelude::*;
use futures::future::{BoxFuture, ready, FutureExt};
use std::any::{Any, type_name};

pub struct QueryComparatorInstance {
    comparator: Box<dyn PackedQueryComparator>,
}

impl QueryComparatorInstance {
    pub fn new<Q, E, C>(comparator: C) -> QueryComparatorInstance 
        where 
            C: QueryComparator<E, Q> + 'static,
            Q: Query,
            E: Any,
    {
        let comparator = move |entity: &dyn Any, query: &dyn Any| -> Result<bool, Failure> {
            let entity = match entity.downcast_ref::<E>() {
                Some(entity) => entity,
                None => {
                    return crate::fail!("Unable to downcast ref entity `{}` for query comparasion", type_name::<E>());
                },
            };
            let query = match query.downcast_ref::<Q>() {
                Some(query) => query,
                None => {
                    return crate::fail!("Unable to downcast ref query `{}` for query comparasion", type_name::<Q>());
                },
            };
            return comparator.compare_entity(entity, query);
        };
        let comparator: Box<dyn PackedQueryComparator> = Box::new(comparator);
        return QueryComparatorInstance {
            comparator,
        }
    }
}