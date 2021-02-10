use std::path::PathBuf;

#[derive(new, Debug)]
pub struct EdgarCacheConfig {
    base_path: PathBuf,
}

#[derive(new, Debug)]
pub struct EdgarCache {
    config: EdgarCacheConfig,
}

impl EdgarCache {
    
}