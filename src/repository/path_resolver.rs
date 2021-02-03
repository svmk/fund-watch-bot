use crate::prelude::*;
use crate::repository::model::identity::Identity;
use std::path::PathBuf;
pub trait PathResolver {
    fn resolve_path(&self, id: &dyn Identity) -> Result<PathBuf, Failure>;
}