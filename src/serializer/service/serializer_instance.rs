use crate::serializer::serializer::Serializer;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::serializer::service::yaml_serializer::YamlSerializer;
use crate::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::io::Read;


#[derive(Debug)]
pub enum SerializerInstance {
    Json(JsonSerializer),
    Yaml(YamlSerializer),
}

impl Serializer for SerializerInstance {
    fn from_slice<T>(&self, data: &[u8]) -> Result<T, Failure>
        where
            T: DeserializeOwned {
                match self {
                    SerializerInstance::Json(ref service) => {
                        return service.from_slice(data);
                    },
                    SerializerInstance::Yaml(ref service) => {
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
                    SerializerInstance::Yaml(ref service) => {
                        return service.to_vec(value);
                    },
                }
            }
    fn from_reader<T>(&self, reader: &mut dyn Read) -> Result<T, Failure>
        where
            T: DeserializeOwned {
                match self {
                    SerializerInstance::Json(ref service) => {
                        return service.from_reader(reader);
                    },
                    SerializerInstance::Yaml(ref service) => {
                        return service.from_reader(reader);
                    },
                }
            }
}