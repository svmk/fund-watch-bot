use crate::prelude::*;
use std::any::Any;

pub trait PackedQueryComparator: Send + Sync {
    fn compare_entity(&self, query: &dyn Any, entity: &dyn Any) -> Result<bool, Failure>;
}

impl <F> PackedQueryComparator for F 
    where 
        F: Fn(&dyn Any, &dyn Any) -> Result<bool, Failure>,
        F: Send + Sync,
{
    fn compare_entity(&self, query: &dyn Any, entity: &dyn Any) -> Result<bool, Failure> {
        return (self)(query, entity);
    }
} 