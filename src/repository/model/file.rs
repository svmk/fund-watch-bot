use crate::repository::model::abs_file::AbsFile;
use crate::repository::model::relative_path::RelativePath;
use async_std::fs::File as AsyncFile;
use std::path::PathBuf;
pub trait File {
    fn new(relative_path: RelativePath, path: PathBuf, file: AsyncFile) -> Self;
    fn resolve_path(&self) -> PathBuf;
}

impl <T>AbsFile for T where T: File {
    fn resolve_abs_path(&self) -> PathBuf {
        return self.resolve_path();
    }
}