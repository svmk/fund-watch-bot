use crate::serializer::serializer::Serializer;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub enum SerializerInstance {
    Json(JsonSerializer),
}

impl Serializer for SerializerInstance {
    fn from_slice<'a, T>(&self, data: &'a [u8]) -> Result<T, Failure>
        where
            T: Deserialize<'a> {
                match self {
                    SerializerInstance::Json(ref service) => {
                        return service.from_slice(data);
                    },
                }
            }

    fn to_vec<T: ?Sized>(&self, value: &T) -> Result<Vec<u8>, Failure> 
        where
            T: Serialize {
                match self {
                    SerializerInstance::Json(ref service) => {
                        return service.to_vec(value);
                    },
                }
            }
}