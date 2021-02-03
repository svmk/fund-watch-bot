use crate::serializer::serializer::Serializer;
use crate::serializer::service::serializer_instance::SerializerInstance;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};

#[derive(Debug)]
pub struct JsonSerializer {} 

impl JsonSerializer {
    pub fn new() -> SerializerInstance {
        return SerializerInstance::Json(JsonSerializer{});
    }
}

impl Serializer for JsonSerializer {
    fn from_slice<'a, T>(&self, data: &'a [u8]) -> Result<T, Failure>
        where
            T: Deserialize<'a> {
                let value: T = from_slice(data)?;
                return Ok(value);
            }

    fn to_vec<T: ?Sized>(&self, value: &T) -> Result<Vec<u8>, Failure> 
        where
        T: Serialize {
            let data = to_vec(value)?;
            return Ok(data);
        }
}