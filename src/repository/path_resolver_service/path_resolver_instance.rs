use crate::repository::path_resolver_service::path_resolver_directory::PathResolverDirectory;
use crate::repository::path_resolver::PathResolver;
use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PathResolverInstance {
    Directory(PathResolverDirectory),
}

impl PathResolver for PathResolverInstance {
    fn base_path(&self) -> Result<PathBuf, Failure> {
        match self {
            PathResolverInstance::Directory(service) => {
                return service.base_path();
            },
        }
    }
    fn resolve_path(&self, id: &RelativePath) -> Result<PathBuf, Failure> {
        match self {
            PathResolverInstance::Directory(service) => {
                return service.resolve_path(id);
            },
        }
    }
}