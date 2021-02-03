use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub trait Serializer {
    fn from_slice<'a, T>(&self, data: &'a [u8]) -> Result<T, Failure>
        where
            T: Deserialize<'a>;

    fn to_vec<T: ?Sized>(&self, value: &T) -> Result<Vec<u8>, Failure> 
        where
            T: Serialize;
}