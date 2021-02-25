use std::any::TypeId;

#[derive(new, Debug)]
pub struct QueryType(TypeId);