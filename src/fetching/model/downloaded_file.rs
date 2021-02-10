use crate::prelude::*;
use crate::fetching::model::url::Url;
use crate::repository::model::abs_file::AbsFile;
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempPath};

#[derive(Debug)]
pub struct DownloadedFile {
    url: Url,
    path: TempPath,
}

impl DownloadedFile {
    pub fn new(url: Url) -> Result<DownloadedFile, Failure> {
        let file = NamedTempFile::new()?;
        let path = file.into_temp_path();
        let downloaded_file = DownloadedFile {
            url,
            path,
        };
        return Ok(downloaded_file);
    }

    pub fn get_path(&self) -> &Path {
        return &self.path;
    }
}

impl AbsFile for DownloadedFile {
    fn resolve_abs_path(&self) -> PathBuf {
        return self.get_path().to_path_buf();
    }   
}