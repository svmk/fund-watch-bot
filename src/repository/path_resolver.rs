use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use std::path::PathBuf;
pub trait PathResolver {
    fn resolve_path(&self, id: &RelativePath) -> Result<PathBuf, Failure>;
}