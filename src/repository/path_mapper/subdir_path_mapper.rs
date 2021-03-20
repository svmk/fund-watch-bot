use crate::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;
use crate::repository::path_mapper::path_mapper::PathMapper;

#[derive(Debug)]
pub struct SubdirPathMapper {
    start_at: usize,
    length: usize,
    fail_when_empty: bool,
}

impl SubdirPathMapper {
    pub fn new(
        start_at: usize,
        length: usize,
    ) -> SubdirPathMapper {
        return SubdirPathMapper {
            start_at,
            length,
            fail_when_empty: false,
        }
    }

    fn map_subdir(&self, directory: &OsStr) -> Result<Option<String>, Failure> {
        let directory = directory.to_string_lossy();
        let mut end_at = self.start_at + self.length;
        if end_at >= directory.len() {
            end_at = directory.len();
        }
        let subdir = directory.get(self.start_at..end_at).unwrap_or_default();
        if subdir.is_empty() {
            if self.fail_when_empty {
                return crate::fail!("Unable to extract subdir from `{}` with range {}..{}", directory, self.start_at, end_at);
            } else {
                return Ok(None);                
            }
        }
        let subdir = subdir.to_string();
        return Ok(Some(subdir));
    }
}

impl PathMapper for SubdirPathMapper {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure> {
        let path = self.map_subdir(path.as_os_str())?;
        let mut result = PathBuf::new();
        if let Some(path) = path {
            result.push(path);
        }
        return Ok(result);
    }
}