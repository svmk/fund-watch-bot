use std::any::TypeId;

#[derive(new, Debug, Hash, PartialEq, Eq)]
pub struct QueryType(TypeId);