use crate::prelude::*;
use std::path::PathBuf;
use crate::repository::path_resolver::path_mapper::PathMapper;

#[derive(new, Debug)]
pub struct FileExtensionMapper {
    extension: String,
}

impl FileExtensionMapper {
    pub fn json() -> FileExtensionMapper {
        return FileExtensionMapper {
            extension: "json".to_string(),
        }
    }
}

impl PathMapper for FileExtensionMapper {
    fn map_path(&self, mut path: PathBuf) -> Result<PathBuf, Failure> {
        path.set_extension(&self.extension);
        return Ok(path);
    }
}