use crate::repository::model::file::File;
use crate::repository::model::relative_path::RelativePath;
use std::path::PathBuf;
use async_std::fs::File as AsyncFile;

#[derive(Debug)]
pub struct EdgarFile {
    file: AsyncFile,
    path: PathBuf,
    relative_path: RelativePath,
}

impl EdgarFile {
    pub fn into_file(self) -> AsyncFile {
        return self.file;
    }

    pub fn get_path(&self) -> &PathBuf {
        return &self.path;
    }
}

impl File for EdgarFile {
    fn new(relative_path: RelativePath, path: PathBuf, file: AsyncFile) -> Self {
        return EdgarFile {
            relative_path,
            path,
            file,
        };
    }

    fn resolve_path(&self) -> PathBuf {
        return self.path.clone();
    }
}