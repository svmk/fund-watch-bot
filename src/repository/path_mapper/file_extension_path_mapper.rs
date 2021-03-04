use crate::prelude::*;
use std::path::PathBuf;
use crate::repository::path_mapper::path_mapper::PathMapper;

#[derive(new, Debug)]
pub struct FileExtensionPathMapper {
    extension: String,
}

impl FileExtensionPathMapper {
    pub fn json() -> FileExtensionPathMapper {
        return FileExtensionPathMapper {
            extension: "json".to_string(),
        }
    }
}

impl PathMapper for FileExtensionPathMapper {
    fn map_path(&self, mut path: PathBuf) -> Result<PathBuf, Failure> {
        path.set_extension(&self.extension);
        return Ok(path);
    }
}