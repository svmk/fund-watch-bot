use crate::prelude::*;
use std::any::Any;

pub trait PackedQueryComparator {
    fn compare_entity(&self, entity: &dyn Any, query: &dyn Any) -> Result<bool, Failure>;
}

impl <F> PackedQueryComparator for F 
    where 
        F: Fn(&dyn Any, &dyn Any) -> Result<bool, Failure>,
{
    fn compare_entity(&self, entity: &dyn Any, query: &dyn Any) -> Result<bool, Failure> {
        return (self)(entity, query);
    }
} 