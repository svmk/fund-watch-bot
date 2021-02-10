use crate::prelude::*;
use crate::repository::model::file::File;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::model::abs_file::AbsFile;
use crate::repository::path_resolver::PathResolver;
use crate::repository::path_resolver_service::path_resolver_instance::PathResolverInstance;
use crate::repository::file_storage::storage_instance::StorageInstance;
use std::marker::PhantomData;
use async_std::path::Path as AsyncPath;
use async_std::fs::File as AsyncFile;
use async_std::fs::OpenOptions as AsyncOpenOptions;

#[derive(Debug)]
pub struct FileStorage<F>
{
    _entity: PhantomData<F>,
    path_resolver: PathResolverInstance,
}

impl <F> FileStorage<F>
    where 
        F: File,
{
    pub fn new(
        path_resolver: PathResolverInstance,
    ) -> StorageInstance<F> {
        let repository = FileStorage {
            _entity: PhantomData {},
            path_resolver,
        };
        return StorageInstance::FileStorage(repository);
    }

    pub async fn read(&self, relative_path: &RelativePath) -> Result<F, Failure> {
        let path = self.path_resolver.resolve_path(relative_path)?;
        let file = AsyncFile::open(&path).await?;
        let file = F::new(relative_path.clone(), path, file);
        return Ok(file);
    }

    pub async fn write(&self, relative_path: &RelativePath) -> Result<F, Failure> {
        let path = self.path_resolver.resolve_path(relative_path)?;
        let mut open_options = AsyncOpenOptions::new();
        open_options.read(true);
        open_options.write(true);
        open_options.append(true);
        open_options.truncate(false);
        open_options.create(true);
        let file = open_options.open(&path).await?;
        let file = F::new(relative_path.clone(), path, file);
        return Ok(file);
    }

    pub async fn exists(&self, path: &RelativePath) -> Result<bool, Failure> {
        let path = self.path_resolver.resolve_path(path)?;
        let async_path = AsyncPath::new(&path);
        return Ok(async_path.exists().await);
    }

    pub async fn replace(&self, path: &RelativePath, file: &dyn AbsFile) -> Result<(), Failure> {
        let path = self.path_resolver.resolve_path(path)?;
        async_std::fs::copy(file.resolve_abs_path(), path).await?;
        return Ok(());
    }
}