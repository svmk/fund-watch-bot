use crate::repository::path_resolver_service::path_resolver_instance::PathResolverInstance;
use crate::repository::path_resolver::PathResolver;
use crate::repository::model::relative_path::RelativePath;
use crate::prelude::*;
use std::path::PathBuf;
use async_std::fs::create_dir_all;

#[derive(Debug)]
struct SubdirectoryRule {
    start: usize,
    end: usize,
}

impl SubdirectoryRule {
    fn resolve_path(&self, id: &String) -> Result<String, Failure> {
        if let Some(text) = id.get(self.start..self.end) {
            let path = text.to_string();
            return Ok(path);
        }
        return Err(Failure::msg(format!("Unable to get subdirectory by index {}..{} from id `{}`", self.start, self.end, id)));
    }
}

#[derive(Debug)]
pub struct PathResolverConfig {
    directory: PathBuf,
    extension: Option<String>,
    subdirectories: Vec<SubdirectoryRule>,
}

impl PathResolverConfig {
    pub fn new(directory: PathBuf) -> PathResolverConfig {
        return PathResolverConfig {
            directory,
            extension: None,
            subdirectories: Vec::new(),
        };
    }

    pub fn with_extension(mut self, extension: String) -> Self {
        self.extension = Some(extension);
        return self;
    }

    pub fn add_subdirectory_substr(mut self, start: usize, end: usize) -> Self {
        assert!(start <= end);
        let subdirectory = SubdirectoryRule {
            start,
            end,
        };
        self.subdirectories.push(subdirectory);
        return self;
    }
}


#[derive(Debug)]
pub struct PathResolverDirectory {
    config: PathResolverConfig,
}

impl PathResolverDirectory {
    pub async fn new(config: PathResolverConfig) -> Result<PathResolverInstance, Failure> {
        create_dir_all(&config.directory).await?;
        let service = PathResolverDirectory {
            config,
        };
        let service = PathResolverInstance::Directory(service);
        return Ok(service);
    }
}

impl PathResolver for PathResolverDirectory {
    fn resolve_path(&self, id: &RelativePath) -> Result<PathBuf, Failure> {
        unimplemented!()
        // let mut result = self.config.directory.clone();
        // let id = id.to_string();
        // for subdirectory in self.config.subdirectories.iter() {
        //     let subdirectory = subdirectory.resolve_path(&id)?;
        //     result.push(subdirectory);
        // }
        // result.push(id);
        // if let Some(ref extension) = self.config.extension {
        //     result.set_extension(extension);
        // }
        // return Ok(result);
    }
}