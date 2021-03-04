use crate::prelude::*;
use std::path::PathBuf;
use crate::repository::path_mapper::path_mapper::PathMapper;

pub struct PathMapperInstance(Box<dyn PathMapper>);

impl PathMapperInstance {
    pub fn new(path_mapper: impl PathMapper + 'static) -> PathMapperInstance {
        let path_mapper = Box::new(path_mapper);
        return PathMapperInstance(path_mapper);
    }
}

impl PathMapper for PathMapperInstance {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure> {
        return self.0.map_path(path);
    }
}