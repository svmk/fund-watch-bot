use crate::serializer::serializer::Serializer;
use crate::serializer::service::serializer_instance::SerializerInstance;
use crate::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::{from_slice, to_vec, from_reader};
use std::io::Read;

#[derive(Debug)]
pub struct YamlSerializer {} 

impl YamlSerializer {
    pub fn new() -> SerializerInstance {
        return SerializerInstance::Yaml(YamlSerializer{});
    }
}

impl Serializer for YamlSerializer {
    fn from_slice<T>(&self, data: &[u8]) -> Result<T, Failure>
        where
            T: DeserializeOwned {
                let value: T = from_slice(data)?;
                return Ok(value);
            }

    fn to_vec<T: ?Sized>(&self, value: &T) -> Result<Vec<u8>, Failure> 
        where
        T: Serialize {
            let data = to_vec(value)?;
            return Ok(data);
        }

    fn from_reader<T>(&self, reader: &mut dyn Read) -> Result<T, Failure>
    where
        T: DeserializeOwned {
            let value: T = from_reader(reader)?;
            return Ok(value);
        }
}