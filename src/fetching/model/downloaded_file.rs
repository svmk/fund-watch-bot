use crate::prelude::*;
use crate::fetching::model::url::Url;
use std::path::Path;
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