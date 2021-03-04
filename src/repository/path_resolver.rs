use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::path_resolver::path_mapper::PathMapper;
use std::path::PathBuf;
use std::fmt;
mod path_mapper;
mod direct_path_mapper;

#[derive(new)]
pub struct PathResolver {
    base_path: PathBuf,
    path_mapper: Box<dyn PathMapper>,
}

impl PathResolver {
    pub fn base_path(&self) -> Result<PathBuf, Failure> {
        let path = self.base_path.clone();
        return Ok(path);
    }

    pub fn resolve_path(&self, path: RelativePath) -> Result<PathBuf, Failure> {
        let path = self.path_mapper.map_path(path)?;
        let path = self.base_path.join(path.into_path_buf());
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