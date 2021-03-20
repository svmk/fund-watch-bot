use crate::prelude::*;
use std::path::PathBuf;
use crate::repository::path_mapper::path_mapper::PathMapper;
pub struct SuffixPathMapper {
    path_mapper: Box<dyn PathMapper>,
    suffix: String,
}

impl SuffixPathMapper {
    pub fn new(path_mapper: impl PathMapper + 'static, suffix: impl Into<String>) -> SuffixPathMapper {
        return SuffixPathMapper {
            path_mapper: Box::new(path_mapper),
            suffix: suffix.into(),
        }
    }
}

impl PathMapper for SuffixPathMapper {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure> {
        let path = self.path_mapper.map_path(path)?;
        let mut path = path.into_os_string();
        path.push(&self.suffix);
        let path = PathBuf::new().join(path);
        return Ok(path);
    }
}
