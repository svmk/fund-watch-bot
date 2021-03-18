use crate::prelude::*;
use std::path::PathBuf;
use crate::repository::path_mapper::path_mapper::PathMapper;
pub struct FileExtensionPathMapper {
    path_mapper: Box<dyn PathMapper>,
    extension: String,
}

impl FileExtensionPathMapper {
    pub fn new(path_mapper: impl PathMapper + 'static, extension: impl Into<String>) -> FileExtensionPathMapper {
        return FileExtensionPathMapper {
            path_mapper: Box::new(path_mapper),
            extension: extension.into(),
        }
    }
    
    pub fn json(path_mapper: impl PathMapper + 'static) -> FileExtensionPathMapper {
        return FileExtensionPathMapper::new(path_mapper, "json");
    }
}

impl PathMapper for FileExtensionPathMapper {
    fn map_path(&self, mut path: PathBuf) -> Result<PathBuf, Failure> {
        path = self.path_mapper.map_path(path)?;
        path.set_extension(&self.extension);
        return Ok(path);
    }
}