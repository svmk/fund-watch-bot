use crate::prelude::*;
use std::path::Path;
use async_std::fs::create_dir_all as async_create_dir_all;

pub async fn create_parent_dir(path: &Path) -> Result<(), Failure> {
    if let Some(dir_path) = path.parent() {
        async_create_dir_all(dir_path).await?;
    }
    return Ok(());
}