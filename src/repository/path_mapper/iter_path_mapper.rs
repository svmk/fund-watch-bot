use crate::prelude::*;
use crate::repository::path_mapper::path_mapper::PathMapper;
use crate::repository::path_mapper::path_mapper_instance::PathMapperInstance;
use std::path::{PathBuf, Path};

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
        for (mapper_id, path_part) in path.iter().enumerate() {
            let path_part = Path::new(path_part);
            let mut path_part = path_part.to_path_buf();
            if let Some(path_mapper) = self.mappers.get(mapper_id) {
                path_part = path_mapper.map_path(path_part)?;
            }
            result.push(path_part);
        }
        return Ok(result);
    }
}