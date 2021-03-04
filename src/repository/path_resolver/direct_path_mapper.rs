use crate::prelude::*;
use std::path::PathBuf;
use crate::repository::path_resolver::path_mapper::PathMapper;

#[derive(new, Debug)]
pub struct DirectPathMapper{}

impl PathMapper for DirectPathMapper {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure> {
        return Ok(path);
    }
}