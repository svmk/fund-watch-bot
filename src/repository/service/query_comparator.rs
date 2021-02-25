use crate::repository::model::query::Query;
use crate::repository::model::query_type::QueryType;
use crate::repository::service_handlers::query_comparator::QueryComparator as QueryComparatorTrait;
use std::collections::HashMap;
use std::any::Any;
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
            C: QueryComparatorTrait<E, Q> + 'static,
            Q: Query,
            E: Any,
        {
            unimplemented!()
        }
}