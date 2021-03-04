use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PathResolver {
    
}

impl PathResolver {
    pub fn base_path(&self) -> Result<PathBuf, Failure> {
        unimplemented!()
    }

    pub fn resolve_path(&self, id: &RelativePath) -> Result<PathBuf, Failure> {
        unimplemented!()
    }
}