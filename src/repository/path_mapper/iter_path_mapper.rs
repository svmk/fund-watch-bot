use crate::prelude::*;
use crate::repository::path_mapper::path_mapper::PathMapper;
use crate::repository::path_mapper::path_mapper_instance::PathMapperInstance;
use std::path::{PathBuf};

pub struct IterPathMapper {
    mappers: Vec<PathMapperInstance>,
}

impl IterPathMapper {
    pub fn new() -> IterPathMapper {
        return IterPathMapper {
            mappers: Vec::new(),
        }
    }

    pub fn push_mapper(mut self, mapper: impl PathMapper + 'static) -> Self {
        let mapper = mapper.into_instance();
        self.mappers.push(mapper);
        return self;
    }
}

impl PathMapper for IterPathMapper {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure> {
        let mut result = PathBuf::new();
        for path_mapper in self.mappers.iter() {
            let path_item = path_mapper.map_path(path.clone())?;
            result.push(path_item);
        }
        result.push(path);
        return Ok(result);
    }
}