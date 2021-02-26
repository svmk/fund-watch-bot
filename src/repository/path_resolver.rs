use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use std::path::PathBuf;
pub trait PathResolver {
    fn base_path(&self) -> Result<PathBuf, Failure>;
    fn resolve_path(&self, id: &RelativePath) -> Result<PathBuf, Failure>;
}