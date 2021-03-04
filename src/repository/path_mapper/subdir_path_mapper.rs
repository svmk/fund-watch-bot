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
            fail_when_empty: true,
        }
    }

    fn map_subdir(&self, directory: &OsStr) -> Result<String, Failure> {
        let directory = directory.to_string_lossy();
        let end_at = self.start_at + self.length;
        let directory = &directory[self.start_at..end_at];
        if self.fail_when_empty && directory.is_empty() {
            return crate::fail!("Unable to extract subdir from `{}`");
        }
        let directory = directory.to_string();
        return Ok(directory);
    }
}

impl PathMapper for SubdirPathMapper {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure> {
        let mut result = PathBuf::new();
        for part in path.iter() {
            let mapped_part = self.map_subdir(part)?;
            result.push(mapped_part);
            result.push(part);
        }
        return Ok(result);
    }
}