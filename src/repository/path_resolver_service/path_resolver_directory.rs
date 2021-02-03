use crate::repository::path_resolver_service::path_resolver_instance::PathResolverInstance;
use crate::repository::path_resolver::PathResolver;
use crate::prelude::*;
use std::path::PathBuf;
use async_std::fs::create_dir_all;
use crate::repository::model::identity::Identity;


#[derive(Debug)]
pub struct PathResolverDirectory {
    directory: PathBuf,
}

impl PathResolverDirectory {
    pub async fn new(directory: PathBuf) -> Result<PathResolverInstance, Failure> {
        create_dir_all(&directory).await?;
        let service = PathResolverDirectory {
            directory,
        };
        let service = PathResolverInstance::Directory(service);
        return Ok(service);
    }
}

impl PathResolver for PathResolverDirectory {
    fn resolve_path(&self, id: &dyn Identity) -> Result<PathBuf, Failure> {
        unimplemented!();
    }
}