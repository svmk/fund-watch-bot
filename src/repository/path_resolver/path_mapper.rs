use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;

pub trait PathMapper: Send + Sync {
    fn map_path(&self, path: RelativePath) -> Result<RelativePath, Failure>;
}