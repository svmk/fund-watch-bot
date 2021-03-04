use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::path_mapper::path_mapper_instance::PathMapperInstance;
use crate::repository::path_mapper::path_mapper::PathMapper;
use std::path::PathBuf;
use std::fmt;

#[derive(new)]
pub struct PathResolver {
    base_path: PathBuf,
    path_mapper: PathMapperInstance,
}

impl PathResolver {
    pub fn base_path(&self) -> Result<PathBuf, Failure> {
        let path = self.base_path.clone();
        return Ok(path);
    }

    pub fn resolve_path(&self, path: RelativePath) -> Result<PathBuf, Failure> {
        let path = self.path_mapper.map_path(path.into_path_buf())?;
        let path = self.base_path.join(path);
        return Ok(path);
    }
}

impl fmt::Debug for PathResolver {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PathResolver")
            .field("base_path", &self.base_path)
            .finish()
    }
}