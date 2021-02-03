use crate::repository::path_resolver_service::path_resolver_directory::PathResolverDirectory;
use crate::repository::path_resolver::PathResolver;
use crate::prelude::*;
use crate::repository::model::identity::Identity;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PathResolverInstance {
    Directory(PathResolverDirectory),
}

impl PathResolver for PathResolverInstance {
    fn resolve_path(&self, id: &dyn Identity) -> Result<PathBuf, Failure> {
        match self {
            PathResolverInstance::Directory(service) => {
                return service.resolve_path(id);
            },
        }
    }
}