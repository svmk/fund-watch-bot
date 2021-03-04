use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::path_resolver::path_mapper::PathMapper;

#[derive(new, Debug)]
pub struct DirectPathMapper{}

impl PathMapper for DirectPathMapper {
    fn map_path(&self, path: RelativePath) -> Result<RelativePath, Failure> {
        return Ok(path);
    }
}