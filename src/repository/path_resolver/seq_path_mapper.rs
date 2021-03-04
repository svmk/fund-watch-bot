use crate::prelude::*;
use crate::repository::path_resolver::path_mapper::PathMapper;
use std::path::{PathBuf, Path};

pub struct SeqPathMapper {
    mappers: Vec<Box<dyn PathMapper>>,
}

impl SeqPathMapper {
    pub fn new() -> SeqPathMapper {
        return SeqPathMapper {
            mappers: Vec::new(),
        }
    }

    pub fn push_mapper(mut self, mapper: impl PathMapper + 'static) -> Self {
        let mapper = Box::new(mapper);
        self.mappers.push(mapper);
        return self;
    }
}

impl PathMapper for SeqPathMapper {
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