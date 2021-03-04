use crate::repository::path_resolver::PathResolver;
use crate::repository::path_mapper::direct_path_mapper::DirectPathMapper;
use std::path::PathBuf;

pub fn edgar_cache_path_resolver(path: PathBuf) -> PathResolver {
    return PathResolver::new(path, DirectPathMapper::new());
}