use crate::prelude::*;
use crate::repository::path_resolver::path_mapper::PathMapper;
use std::path::PathBuf;

pub struct IterPathMapper {
    mappers: Vec<Box<dyn PathMapper>>,
}

impl IterPathMapper {
    pub fn new() -> IterPathMapper {
        return IterPathMapper {
            mappers: Vec::new(),
        }
    }

    pub fn push_mapper(mut self, mapper: impl PathMapper + 'static) -> Self {
        let mapper = Box::new(mapper);
        self.mappers.push(mapper);
        return self;
    }
}

impl PathMapper for IterPathMapper {
    fn map_path(&self, mut path: PathBuf) -> Result<PathBuf, Failure> {
        for path_mapper in self.mappers.iter() {
            path = path_mapper.map_path(path)?;
        }
        return Ok(path);
    }
}