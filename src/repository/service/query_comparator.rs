use crate::repository::model::query::Query;
use crate::repository::model::query_type::QueryType;
use crate::repository::service_handlers::query_comparator::QueryComparator as QueryComparatorTrait;
use crate::prelude::*;
use std::collections::HashMap;
use std::any::{Any, type_name};
mod packed_query_comparator;
mod query_comparator_instance;
use self::query_comparator_instance::QueryComparatorInstance;

pub struct QueryComparator {
    comparators: HashMap<QueryType, QueryComparatorInstance>,
}

impl QueryComparator {
    pub fn new() -> QueryComparator {
        return QueryComparator {
            comparators: HashMap::new(),
        }
    }

    pub fn register<E, Q, C>(&mut self, comparator: C) 
        where 
            C: QueryComparatorTrait<Q, E> + 'static,
            Q: Query,
            E: Any,
        {
            let comparator = QueryComparatorInstance::new(comparator);
            let query_type = Q::get_query_type();
            let _ = self.comparators.insert(query_type, comparator);
        }

    pub fn compare_entity<E, Q>(&self, query: &Q, entity: &E) -> Result<bool, Failure> 
        where 
            Q: Query,
            E: Any,
    {
        let query_type = Q::get_query_type();
        if let Some(comparator) = self.comparators.get(&query_type) {
            return comparator.compare(query, entity);
        }
        return crate::fail!("Unable to get entity comparator for query `{}`", type_name::<Q>());
    }
}