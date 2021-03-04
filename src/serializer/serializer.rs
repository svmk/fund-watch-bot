use crate::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::io::Read;

pub trait Serializer {
    fn from_slice<T>(&self, data: &[u8]) -> Result<T, Failure>
        where
            T: DeserializeOwned;

    fn to_vec<T: ?Sized>(&self, model: &T) -> Result<Vec<u8>, Failure> 
        where
            T: Serialize;
        
    fn from_reader<T>(&self, reader: &mut dyn Read) -> Result<T, Failure>
        where
            T: DeserializeOwned;
}