use crate::repository::path_resolver::PathResolver;
use crate::repository::path_mapper::file_extension_path_mapper::FileExtensionPathMapper;
use crate::repository::path_mapper::subdir_path_mapper::SubdirPathMapper;
use crate::repository::path_mapper::iter_path_mapper::IterPathMapper;
use std::path::PathBuf;

pub fn cusip_cache_path_resolver(mut path: PathBuf) -> PathResolver {
    path.push("cusip_cache");
    let path_mapper = IterPathMapper::new()
        .push_mapper(SubdirPathMapper::new(0, 2))
        .push_mapper(SubdirPathMapper::new(2, 2));
    let path_mapper = FileExtensionPathMapper::json(path_mapper);
    return PathResolver::new(path, path_mapper);
}