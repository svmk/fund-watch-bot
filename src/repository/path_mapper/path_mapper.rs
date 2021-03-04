use crate::prelude::*;
use crate::repository::path_mapper::path_mapper_instance::PathMapperInstance;
use std::path::PathBuf;

pub trait PathMapper: Send + Sync {
    fn into_instance(self) -> PathMapperInstance 
        where Self: Sized + 'static 
    {
        return PathMapperInstance::new(self);
    }
    
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure>;
}