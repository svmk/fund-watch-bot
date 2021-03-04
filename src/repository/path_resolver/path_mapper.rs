use crate::prelude::*;
use std::path::PathBuf;

pub trait PathMapper: Send + Sync {
    fn map_path(&self, path: PathBuf) -> Result<PathBuf, Failure>;
}